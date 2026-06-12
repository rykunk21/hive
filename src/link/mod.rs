//! Link / communication layer — inter-pod and hive-pod messaging.
//!
//! The link layer bridges actix actor messages with the hive's tokio-based
//! orchestration loop. Pod actors send results back to the hive via the
//! [`MessageRouter`], which the hive polls in its main loop.
//!
//! ## Architecture
//!
//! - Pod actors ([`PodActor`]) send [`PodMessage`] to their [`MessageRouter`] handle
//! - The hive polls `router.recv()` in a tokio select loop
//! - Hive dispatches responses to the appropriate handler (result aggregation,
//!   knowledge base updates, TUI refresh)
//!
//! This decouples the actix actor runtime from the hive's async orchestration.

use tokio::sync::mpsc;

// ---------------------------------------------------------------------------
// Messages sent FROM pod actors TO the hive
// ---------------------------------------------------------------------------

/// Messages that pod actors send back to the hive.
///
/// These travel through the [`MessageRouter`] and are consumed by the
/// hive's main orchestration loop.
///
/// TODO: Define all response variants.
pub enum PodMessage {
    // TODO: TaskResult { task_id, pod_id, result } — pod completed a task
    // TODO: StatusUpdate { pod_id, status } — pod reports state change
    // TODO: KnowledgeContribution { pod_id, entries } — pod wants to index outputs
    // TODO: Error { pod_id, error } — pod encountered a fatal error
}

// ---------------------------------------------------------------------------
// Message Router
// ---------------------------------------------------------------------------

/// Central message router that collects pod actor outputs.
///
/// Each [`PodActor`] holds a clone of the router's [`Sender`] to send [`PodMessage`]
/// back to the hive. The hive owns the [`Receiver`] and polls it.
///
/// This replaces the earlier mpsc-per-pod design with a single multi-producer
/// channel — simpler and sufficient for hive ↔ pod communication.
///
/// TODO: Implement Sender/Receiver management and optional backpressure.
pub struct MessageRouter {
    // TODO: tx: mpsc::Sender<PodMessage> — cloned and given to each pod actor
    // TODO: rx: mpsc::Receiver<PodMessage> — owned by hive, polled in main loop
    // TODO: backpressure: BackpressureConfig — queue limits and overflow handling
}

impl MessageRouter {
    /// Create a new message router with bounded capacity.
    ///
    /// TODO: Create bounded channel with configurable capacity.
    pub fn new() -> Self {
        MessageRouter {}
    }

    /// Get a sender handle to give to a newly spawned pod actor.
    ///
    /// The actor stores this and uses it to send [`PodMessage`] back to
    /// the hive. The sender is cheaply cloneable.
    ///
    /// TODO: Return cloned sender.
    pub fn sender(&self,
    ) -> mpsc::Sender<PodMessage> {
        todo!("clone and return the router's sender handle")
    }

    /// Poll for the next message from pod actors.
    ///
    /// The hive calls this in its async main loop. Returns `None` if the
    /// channel is closed (all senders dropped).
    ///
    /// TODO: Return next PodMessage or None if channel closed.
    pub async fn recv(
        &mut self,
    ) -> Option<PodMessage> {
        todo!("await next message from the receiver")
    }

    /// Try to receive a message without awaiting.
    ///
    /// Used by the TUI tick to check for pending messages without blocking.
    ///
    /// TODO: Return next PodMessage or None if empty.
    pub fn try_recv(
        &mut self,
    ) -> Option<PodMessage> {
        todo!("non-blocking poll of the receiver")
    }
}

/// Configuration for backpressure behavior.
///
/// TODO: Define queue capacity limits, timeout, and overflow strategy.
pub struct BackpressureConfig;
