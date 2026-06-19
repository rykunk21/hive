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

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

/// Type of message that pod can recieve from hive
#[derive(Message)]
#[rtype(result = "PodResponse")]
pub enum PodMessage {
    Kill,
    Replicate,
    Query(String),
}

/// Type of messages that pod can return to hive
pub enum PodResponse {
    Ok(String),
    Err,
}

impl<A, M> MessageResponse<A, M> for PodResponse
where
    A: Actor,
    M: Message<Result = PodResponse>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}
