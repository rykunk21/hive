use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

/// Type of message that ping can recieve
#[derive(Message)]
#[rtype(result = "PingResponses")]
pub enum PingMessages {
    Ping,
    Pong,
}

/// Type of messages that ping can return
pub enum PingResponses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for PingResponses
where
    A: Actor,
    M: Message<Result = PingResponses>,
{
    fn handle(self, _ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            let _ = tx.send(self);
        }
    }
}

// Define actor
pub struct Ping;

// Provide Actor implementation for our actor
impl Actor for Ping {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}

    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

/// Define handler for `Messages` enum
impl Handler<PingMessages> for Ping {
    type Result = PingResponses;

    fn handle(&mut self, msg: PingMessages, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PingMessages::Ping => PingResponses::GotPing,
            PingMessages::Pong => PingResponses::GotPong,
        }
    }
}
