//! Pod runtime — typed agent units that execute within a Hive.
//!
//! Pods are the worker agents of the hive. Each pod has a type that declares
//! its domain, retrieval preferences, and resource requirements. Pods run
//! asynchronously and communicate with the hive via structured messages.

/// Trait that all pods must implement to participate in a Hive runtime.
///
/// The hive constructs pods through a registry of type-name → constructor
/// mappings. Once spawned, a pod runs its own async loop, receiving tasks
/// and sending results back to the hive.
///
/// TODO: Define the full lifecycle and communication protocol.
pub trait Pod: Send {
    /// Initialize the pod before it begins processing tasks.
    ///
    /// The hive calls this once after construction and before the first task.
    /// Pods should load any type-specific resources or warm caches here.
    ///
    /// TODO: Accept initialization context (config handles, knowledge base refs).
    fn init(&mut self) -> anyhow::Result<()> {
        todo!("initialize pod with type-specific resources")
    }

    /// Execute the pod's main processing loop.
    ///
    /// The hive runs this on a tokio task. The pod should:
    /// - Listen for incoming tasks from the hive
    /// - Process each task using domain-specific logic
    /// - Emit results and any knowledge contributions back to the hive
    /// - Handle shutdown signals gracefully
    ///
    /// TODO: Define task and result message types.
    fn run(&mut self) -> anyhow::Result<()> {
        todo!("run async pod loop: receive tasks, process, emit results")
    }

    /// Shut down the pod cleanly.
    ///
    /// The hive signals this when the pod is being killed or the system
    /// is shutting down. Pods should flush any pending state and exit.
    ///
    /// TODO: Implement graceful termination with timeout.
    fn shutdown(&mut self) -> anyhow::Result<()> {
        todo!("flush state and exit gracefully")
    }

    /// Return metadata about this pod's type.
    ///
    /// Used by the hive to select retrieval strategies and routing behavior.
    ///
    /// TODO: Return PodType with retrieval preferences and resource budget.
    fn pod_type(&self) -> PodType {
        todo!("return static type metadata for this pod implementation")
    }
}

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

/// Registry that maps type names to pod constructors.
///
/// Built-in pod types register at startup. External crates register by
/// providing a constructor closure that the hive calls when spawning.
///
/// TODO: Implement HashMap-based registry with constructor closures.
pub struct PodRegistry;

impl PodRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        PodRegistry
    }

    /// Register a built-in or external pod type.
    ///
    /// TODO: Accept type name + constructor closure.
    pub fn register(&mut self, _type_name: &str) {
        todo!("register a pod constructor for the given type name")
    }

    /// Look up a pod type by name.
    ///
    /// TODO: Return constructor if found.
    pub fn get(&self, _type_name: &str) -> Option<()> {
        todo!("return pod constructor for the given type name")
    }

    /// Return all registered type names.
    ///
    /// TODO: Return sorted list of names.
    pub fn type_names(&self) -> Vec<String> {
        todo!("return all registered type names")
    }
}

/// A running pod handle used by the hive for lifecycle management.
///
/// TODO: Include the tokio task handle, message channel, and status.
pub struct RunningPod;

/// Resource budget allocated to a pod type.
///
/// TODO: Define CPU, memory, and LLM token limits.
pub struct ResourceBudget;
