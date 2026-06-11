//! Link / communication layer — inter-pod and hive-pod messaging.
//!
//! The link layer provides async message passing between pods and the hive.
/// It handles result aggregation, backpressure, and message routing.

/// Message types for hive-pod and pod-pod communication.
///
/// TODO: Define task assignments, results, status updates, and control signals.
pub enum Message {
    // TODO: Task { task_id, pod_type, payload } — hive assigns work to a pod
    // TODO: Result { task_id, pod_id, output } — pod reports completion
    // TODO: Status { pod_id, state } — pod reports current state (running, idle, error)
    // TODO: Knowledge { pod_id, entries } — pod contributes to knowledge base
    // TODO: Shutdown { pod_id } — hive signals pod to terminate
    // TODO: Query { pod_type, query, response_tx } — pod asks hive to query knowledge base
}

/// Message router owned by the hive.
///
/// Maintains channels to all active pods and routes messages between them.
/// Also implements backpressure: if a pod's channel is full, the router
/// can throttle upstream senders or queue messages.
///
/// TODO: Implement channel management and routing logic.
pub struct MessageRouter {
    // TODO: channels: HashMap<PodId, mpsc::Sender<Message>> — outbound channels to pods
    // TODO: event_rx: mpsc::Receiver<Message> — inbound events from all pods
    // TODO: backpressure: BackpressureConfig — queue limits and throttling rules
}

impl MessageRouter {
    /// Create a new message router.
    ///
    /// TODO: Initialize with empty channel map.
    pub fn new() -> Self {
        MessageRouter
    }

    /// Register a new pod's communication channel.
    ///
    /// Called when the hive spawns a pod. Creates the pod's inbound channel
    /// and returns the sender for the hive to use.
    ///
    /// TODO: Create channel, store sender, spawn listener for pod events.
    pub fn register_pod(
        &mut self,
        _pod_id: PodId,
    ) -> anyhow::Result<mpsc::Sender<Message>> {
        todo!("create channel for pod and register with router")
    }

    /// Unregister a pod and close its channel.
    ///
    /// TODO: Remove sender and signal any pending messages.
    pub fn unregister_pod(&mut self,
        _pod_id: PodId,
    ) -> anyhow::Result<()> {
        todo!("close pod channel and remove from router")
    }

    /// Route a message to a specific pod.
    ///
    /// TODO: Look up channel, apply backpressure rules, and send.
    pub fn send_to(
        &self,
        _pod_id: PodId,
        _message: Message,
    ) -> anyhow::Result<()> {
        todo!("route message to specific pod with backpressure")
    }

    /// Broadcast a message to all active pods of a given type.
    ///
    /// Used by the hive to fan out tasks to all pods of a type.
    ///
    /// TODO: Filter pods by type and send to each.
    pub fn broadcast_to_type(
        &self,
        _pod_type: &str,
        _message: Message,
    ) -> anyhow::Result<()> {
        todo!("broadcast message to all pods of a type")
    }

    /// Collect incoming events from all pods.
    ///
    /// The hive calls this in its main loop to process pod results and status.
    ///
    /// TODO: Poll event_rx and return next message.
    pub fn recv(&mut self) -> anyhow::Result<Option<Message>> {
        todo!("receive next event from pod event channel")
    }
}

/// Unique identifier for a pod instance.
///
/// TODO: Re-export or share with hive::PodId.
pub struct PodId(String);

/// Configuration for backpressure behavior.
///
/// TODO: Define queue capacity limits, timeout, and overflow strategy.
pub struct BackpressureConfig;

use tokio::sync::mpsc;
