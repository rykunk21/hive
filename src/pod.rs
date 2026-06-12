//! Pod runtime — typed agent units that execute within a Hive.
//!
//! Pods are the worker agents of the hive. Each pod has a type that declares
//! its domain, retrieval preferences, and resource requirements. Pods run
//! asynchronously and communicate with the hive via structured messages.
//!
//! ## Actor Model (faithful to actix patterns)
//!
//! Every pod is an actix::Actor. Consumers implement the [`Pod`] trait on
//! a plain struct; the [`PodActor`] wrapper bridges trait methods to the actor
//! lifecycle. The hive interacts with pods only through actix::Addr.
//!
//! Messages follow actix conventions:
//! - Derive `Message` with `#[rtype(result = "...")]`
//! - Handler returns `Self::Result` directly (sync) or `ResponseActFuture` (async)
//! - Actor lifecycle: started() → running → stopping() → stopped()

use actix::prelude::*;

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

/// Assign a task to a pod.
///
/// Sent by the hive to a [`PodActor`]. The actor delegates to
/// `Pod::handle_task` and returns the result.
#[derive(Message)]
#[rtype(result = "anyhow::Result<TaskResult>")]
pub struct Task {
    /// Text payload for the task (e.g., user message for a speaker pod).
    pub text: String,
    /// Optional channel/source identifier for routing responses back.
    pub channel: Option<String>,
}

impl Task {
    /// Create a simple text task.
    pub fn text(input: impl Into<String>) -> Self {
        Task {
            text: input.into(),
            channel: None,
        }
    }

    /// Create a task tied to a specific channel (e.g., Discord channel ID).
    pub fn chat(input: impl Into<String>, channel: impl Into<String>) -> Self {
        Task {
            text: input.into(),
            channel: Some(channel.into()),
        }
    }
}

/// Signal a pod to shut down.
#[derive(Message)]
#[rtype(result = "()")]
pub struct Shutdown;

/// Query the current status of a pod.
#[derive(Message)]
#[rtype(result = "()")]
pub struct QueryStatus;

// ---------------------------------------------------------------------------
// Result and status types
// ---------------------------------------------------------------------------

/// Result of processing a [`Task`].
#[derive(Debug, Clone)]
pub struct TaskResult {
    /// Response text from the pod (e.g., chat reply).
    pub response: String,
    /// If set, hive should send this response back to the given channel.
    pub reply_to: Option<String>,
}

impl TaskResult {
    /// Create a simple text result with no routing.
    pub fn text(response: impl Into<String>) -> Self {
        TaskResult {
            response: response.into(),
            reply_to: None,
        }
    }

    /// Create a result that routes back to a specific channel.
    pub fn reply(response: impl Into<String>, channel: impl Into<String>) -> Self {
        TaskResult {
            response: response.into(),
            reply_to: Some(channel.into()),
        }
    }
}

/// Snapshot of a pod's current status.
pub struct PodStatus;

/// Metadata describing a pod type for hive registration.
pub struct PodType {
    /// Unique type name (e.g., "planner", "summarizer", "speaker").
    pub name: String,
}

// ---------------------------------------------------------------------------
// Pod trait — consumer-facing interface
// ---------------------------------------------------------------------------

/// Trait that all pods must implement to participate in a Hive runtime.
///
/// Consumers implement this trait on a plain struct with no knowledge of
/// actix internals. The [`PodActor`] wrapper translates actix messages into
/// trait method invocations.
pub trait Pod: Send + Unpin + 'static {
    /// Initialize the pod before it begins processing tasks.
    fn init(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Process a single task and return the result.
    ///
    /// This is called by the Handler<Task> implementation in PodActor.
    /// The pod can perform async work (LLM calls, I/O) by having the
    /// Handler spawn a future and await it.
    fn handle_task(
        &mut self,
        task: Task,
    ) -> anyhow::Result<TaskResult>;

    /// Shut down the pod cleanly.
    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Return metadata about this pod's type.
    fn pod_type(&self) -> PodType;
}

// ---------------------------------------------------------------------------
// PodActor — actix wrapper
// ---------------------------------------------------------------------------

/// Actix actor that wraps a [`Pod`] implementation.
///
/// The hive never interacts with [`Pod`] directly — it only sends messages
/// to a [`PodActor`]'s [`Addr`].
pub struct PodActor<P: Pod> {
    /// The wrapped pod instance. Accessible to custom Handlers that need
    /// pod-specific state (e.g., the SpeakerPod's Ollama client).
    pub pod: P,
}

impl<P: Pod> PodActor<P> {
    /// Wrap a concrete pod in an actor.
    pub fn new(pod: P) -> Self {
        PodActor { pod }
    }

    /// Start this actor and return its address.
    pub fn start_for(pod: P) -> Addr<Self> {
        PodActor::new(pod).start()
    }
}

impl<P: Pod> Actor for PodActor<P> {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if let Err(e) = self.pod.init() {
            log::error!("pod init failed: {}", e);
            ctx.stop();
        }
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        let _ = self.pod.shutdown();
        Running::Stop
    }
}

// ---------------------------------------------------------------------------
// Handler implementations — faithful to actix patterns
// ---------------------------------------------------------------------------

/// NOTE: No blanket Handler<Task> here. Each pod type that needs async
/// handling provides its own Handler<Task> implementation (see speaker/mod.rs).
/// Sync pods can add a simple Handler<Task> that delegates to Pod::handle_task.

/// Handle [`Shutdown`] by delegating to `Pod::shutdown`.
impl<P: Pod> Handler<Shutdown> for PodActor<P> {
    type Result = ();

    fn handle(&mut self,
        _msg: Shutdown,
        ctx: &mut Self::Context,
    ) {
        let _ = self.pod.shutdown();
        ctx.stop();
    }
}

/// Handle [`QueryStatus`] by returning a snapshot.
impl<P: Pod> Handler<QueryStatus> for PodActor<P> {
    type Result = ();

    fn handle(&mut self,
        _msg: QueryStatus,
        _ctx: &mut Self::Context,
    ) {
        // TODO: return PodStatus
    }
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// Registry that maps type names to pod constructors.
pub struct PodRegistry {
    // TODO: constructors: HashMap<String, Box<dyn Fn() -> Box<dyn Pod>>>
}

impl PodRegistry {
    pub fn new() -> Self {
        PodRegistry {}
    }

    pub fn register(
        &mut self,
        _type_name: &str,
        _constructor: Box<dyn Fn() -> Box<dyn Pod>>,
    ) {
        todo!("store constructor")
    }
}

/// Resource budget allocated to a pod type.
pub struct ResourceBudget;
