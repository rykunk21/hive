use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

/// Type of message that LLM can recieve
#[derive(Message)]
#[rtype(result = "PingResponses")]
pub enum PingMessages {
    Ping,
    Pong,
}

/// Type of messages that LLM can return
pub enum PingResponses {
    GotPing,
    GotPong,
}

impl<A, M> MessageResponse<A, M> for PingResponses
where
    A: Actor,
    M: Message<Result = PingResponses>,
{
    fn handle(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

// Define actor
pub struct Ping;

// Provide Actor implementation for our actor
impl Actor for Ping {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        println!("Actor is stopped");
    }
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
