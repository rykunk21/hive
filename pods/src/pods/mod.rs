//! Built-in pod types.
//!

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
    Response(String),
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

mod genome;
pub mod ping;
pub mod pod;
pub mod speaker;
