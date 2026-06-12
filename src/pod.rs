//! Pod runtime — typed agent units that execute within a Hive.
//!
//! Pods are the worker agents of the hive. Each pod has a type that declares
//! its domain, retrieval preferences, and resource requirements. Pods run
//! asynchronously and communicate with the hive via structured messages.
//!
//! ## Actor Model
//!
//! Every pod is an actix::Actor. Consumers implement the [`Pod`] trait on
//! a plain struct; the [`PodActor`] wrapper bridges trait methods to the actor
//! lifecycle. The hive interacts with pods only through actix::Addr.
//!
//! The trait is split into two layers:
//! - [`Pod`] — the consumer-facing trait (simple methods, no actix types)
//! - [`PodActor`] — wraps a Pod and implements actix::Actor

use actix::prelude::*;
use std::pin::Pin;
use std::marker::Unpin;

/// A boxed future returned by pod task handlers.
pub type TaskFuture = Pin<Box<dyn std::future::Future<Output = anyhow::Result<TaskResult>> + Send>>;

// ---------------------------------------------------------------------------
// Pod trait — consumer-facing interface
// ---------------------------------------------------------------------------

/// Trait that all pods must implement to participate in a Hive runtime.
///
/// Consumers implement this trait on a plain struct with no knowledge of
// actix internals. The [`PodActor`] wrapper translates actix messages into
/// trait method invocations.
///
/// # Lifecycle
///
/// 1. `init` — called once after actor construction, before any tasks.
/// 2. `handle_task` — called repeatedly for each incoming [`Task`] message.
/// 3. `shutdown` — called when the actor receives [`Shutdown`] or system stops.
///
/// # Required Methods
///
/// Every pod must provide:
/// - A constructor (registered via [`PodRegistry`])
/// - Type metadata (`pod_type`)
/// - Task handling logic (`handle_task`)
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Default)]
/// struct PlannerPod;
///
/// impl Pod for PlannerPod {
///     fn pod_type(&self) -> PodType {
///         PodType { name: "planner".into() }
///     }
///
///     fn handle_task(&mut self, _task: Task) -> anyhow::Result<TaskResult> {
///         todo!("plan something")
///     }
/// }
/// ```
pub trait Pod: Send + Unpin + 'static {
    /// Initialize the pod before it begins processing tasks.
    ///
    /// Called once by [`PodActor`] in its `started` hook. Pods should load
    /// type-specific resources, warm caches, or establish connections here.
    ///
    /// TODO: Accept initialization context (config handles, knowledge base refs).
    fn init(&mut self) -> anyhow::Result<()> {
        todo!("initialize pod with type-specific resources")
    }

    /// Process a single task.
    ///
    /// This is the core work method. The actor calls it for every [`Task`]
    /// message received. The pod performs its domain logic (which may include
    /// async LLM calls) and returns a [`TaskResult`] containing outputs.
    ///
    /// This method returns a boxed future so pods can `.await` LLM completions,
    /// I/O, or other async operations without blocking the actor thread.
    fn handle_task(
        &mut self,
        _task: Task,
    ) -> TaskFuture;

    /// Shut down the pod cleanly.
    ///
    /// Called when the actor receives a [`Shutdown`] message or the actix
    /// system stops. Pods should flush pending state, close connections,
    /// and prepare for termination.
    ///
    /// TODO: Implement graceful termination with timeout.
    fn shutdown(&mut self) -> anyhow::Result<()> {
        todo!("flush state and exit gracefully")
    }

    /// Return metadata about this pod's type.
    ///
    /// Used by the hive to select retrieval strategies, resource budgets,
    /// and routing rules. This is static metadata — it does not change
    /// over the pod's lifetime.
    ///
    /// TODO: Return full PodType with retrieval preferences and resource budget.
    fn pod_type(&self) -> PodType;
}

// ---------------------------------------------------------------------------
// PodActor — actix wrapper that bridges Pod trait to actor lifecycle
// ---------------------------------------------------------------------------

/// Actix actor that wraps a [`Pod`] implementation.
///
/// [`PodActor`] is the bridge between the consumer's [`Pod`] trait and the
/// actix runtime. It owns the concrete pod instance and handles all actor
/// lifecycle hooks (`started`, `stopping`, `stopped`) by delegating to
/// the wrapped pod's methods.
///
/// The hive never interacts with [`Pod`] directly — it only sends messages
/// to a [`PodActor`]'s [`Addr`].
///
/// # Type Parameters
///
/// - `P`: The concrete [`Pod`] implementation wrapped by this actor.
///
/// # Messages Handled
///
/// - [`Task`] → calls `Pod::handle_task`, returns [`TaskResult`]
/// - [`Shutdown`] → calls `Pod::shutdown`, then stops the actor
/// - [`QueryStatus`] → returns current pod status snapshot
///
/// TODO: Implement Handler for each message type.
pub struct PodActor<P: Pod> {
    pod: P,
    // TODO: status: PodStatus — running, idle, error
    // TODO: task_count: u64 — total tasks processed
    // TODO: router_tx: mpsc::Sender<PodMessage> — outbound channel to hive MessageRouter
}

impl<P: Pod> PodActor<P> {
    /// Wrap a concrete pod in an actor.
    ///
    /// TODO: Accept the pod instance and communication channels.
    pub fn new(pod: P) -> Self {
        PodActor { pod }
    }

    /// Start this actor and return its address.
    pub fn start_for(pod: P) -> Addr<Self> {
        PodActor::new(pod).start()
    }
}

impl<P: Pod> Actor for PodActor<P> {
    /// Actor context type. Standard actix context.
    type Context = Context<Self>;

    /// Called when the actor starts.
    fn started(&mut self, ctx: &mut Self::Context) {
        if let Err(e) = self.pod.init() {
            log::error!("pod init failed: {}", e);
            ctx.stop();
        }
    }

    /// Called when the actor is stopping.
    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        let _ = self.pod.shutdown();
        Running::Stop
    }
}

// ---------------------------------------------------------------------------
// Actor Messages
// ---------------------------------------------------------------------------

/// Assign a task to a pod.
///
/// Sent by the hive to a [`PodActor`]. The actor delegates to
/// `Pod::handle_task` and returns the result.
///
/// TODO: Expand payload with task_id, context handles, routing metadata.
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

impl Message for Task {
    type Result = anyhow::Result<TaskResult>;
}

/// Signal a pod to shut down.
///
/// Sent by the hive when killing a pod or during system shutdown.
/// The actor calls `Pod::shutdown` and then stops itself.
///
/// TODO: Include graceful shutdown timeout.
pub struct Shutdown;

impl Message for Shutdown {
    type Result = ();
}

/// Query the current status of a pod.
///
/// Sent by the hive (e.g., for TUI display) to check if a pod is
/// running, idle, or in error state.
///
/// TODO: Return PodStatus snapshot.
pub struct QueryStatus;

impl Message for QueryStatus {
    /// Using () for now; real implementation will return a serializable status.
    type Result = ();
}

// ---------------------------------------------------------------------------
// Handler implementations (delegation to Pod trait)
// ---------------------------------------------------------------------------

/// Handle [`Task`] messages by delegating to `Pod::handle_task`.
///
/// The handler is async — it calls `pod.handle_task().await` so the pod
/// can perform async LLM completions without blocking the actor thread.
impl<P: Pod> Handler<Task> for PodActor<P> {
    type Result = ResponseActFuture<Self, anyhow::Result<TaskResult>>;

    fn handle(
        &mut self,
        msg: Task,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let fut = self.pod.handle_task(msg);
        Box::pin(async move {
            fut.await
        }.into_actor(self))
    }
}

/// Handle [`Shutdown`] by delegating to `Pod::shutdown`.
impl<P: Pod> Handler<Shutdown> for PodActor<P> {
    type Result = ();

    fn handle(
        &mut self,
        _msg: Shutdown,
        ctx: &mut Self::Context,
    ) {
        let _ = self.pod.shutdown();
        ctx.stop();
    }
}

/// Handle [`QueryStatus`] by returning a snapshot.
///
/// TODO: Build PodStatus from actor state.
impl<P: Pod> Handler<QueryStatus> for PodActor<P> {
    type Result = ();

    fn handle(
        &mut self,
        _msg: QueryStatus,
        _ctx: &mut Self::Context,
    ) {
        todo!("return current pod status snapshot")
    }
}

// ---------------------------------------------------------------------------
// Result and status types
// ---------------------------------------------------------------------------

/// Result of processing a [`Task`].
///
/// Returned by `Pod::handle_task` and propagated back to the hive.
/// If the task included a `channel`, the hive routes the response there.
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
///
/// Used for TUI display and hive health monitoring.
///
/// TODO: Include state (running/idle/error), task count, resource usage.
pub struct PodStatus;

// ---------------------------------------------------------------------------
// Pod type metadata
// ---------------------------------------------------------------------------

/// Metadata describing a pod type for hive registration.
///
/// The hive uses this to configure per-type retrieval, resource limits,
/// and routing rules.
///
/// TODO: Add retrieval preferences, embedding model, resource budget fields.
pub struct PodType {
    /// Unique type name (e.g., "planner", "summarizer", "code-gen").
    pub name: String,
    // TODO: retrieval_preferences: RetrievalConfig
    // TODO: embedding_model: String — which model to use for indexing this type's outputs
    // TODO: resource_budget: ResourceBudget — CPU, memory, LLM token limits
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// Registry that maps type names to pod constructors.
///
/// Built-in pod types register at startup. External crates register by
/// providing a constructor closure that returns a boxed [`Pod`]. The hive
/// then wraps the result in [`PodActor::start_for`] when spawning.
///
/// TODO: Implement HashMap-based registry with Box<dyn Pod> constructors.
pub struct PodRegistry {
    // TODO: constructors: HashMap<String, Box<dyn Fn() -> Box<dyn Pod>>>
}

impl PodRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        PodRegistry {}
    }

    /// Register a pod type with its constructor.
    ///
    /// The constructor returns a boxed [`Pod`] that the hive will wrap
    /// in a [`PodActor`] at spawn time.
    ///
    /// TODO: Accept type name + constructor closure returning Box<dyn Pod>.
    pub fn register(
        &mut self,
        _type_name: &str,
        _constructor: Box<dyn Fn() -> Box<dyn Pod>>,
    ) {
        todo!("store constructor closure keyed by type name")
    }

    /// Look up a pod type by name and construct an instance.
    ///
    /// The hive calls this during `spawn_pod`, then wraps the result
    /// in [`PodActor::start_for`].
    ///
    /// TODO: Return Box<dyn Pod> if found.
    pub fn construct(
        &self,
        _type_name: &str,
    ) -> Option<Box<dyn Pod>> {
        todo!("look up constructor, invoke it, return boxed pod")
    }

    /// Return all registered type names.
    ///
    /// TODO: Return sorted list of names.
    pub fn type_names(&self) -> Vec<String> {
        todo!("return all registered type names")
    }
}

/// Resource budget allocated to a pod type.
///
/// TODO: Define CPU, memory, and LLM token limits.
pub struct ResourceBudget;
