//! Pod actor module.
//!
//! Defines the `Pod` actor and its message protocol (`PodMessage` / `PodResponse`).
//! Pods are evolvable units that carry a `Genome` and can respond to queries
//! from the hive, replicate, or be shut down.

use std::pin::Pin;

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

mod genome;
mod providers;
use genome::Genome;

use crate::pod::providers::Provider;

type TaskFuture = Pin<Box<dyn Future<Output = anyhow::Result<String>>>>;

const N_INPUTS: usize = 1;
const N_NODES: usize = 8;
const N_OUTPUTS: usize = 1;
const N_FUNCS: usize = 4; // expand as you define real functions

/// Type of message that pod can receive from hive
#[derive(Message)]
#[rtype(result = "PodResponse")]
pub enum PodMessage {
    Kill,
    Replicate,
    Query(String),
}

/// Type of messages that pod can return to hive
#[derive(Message)]
#[rtype(result = "()")]
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

/// A Pod actor with an evolved genome
#[derive(Default)]
pub struct Pod {
    pub genome: Genome,
    pub generation: u32,
    provider: Provider,
}

impl Pod {
    pub fn new() -> Self {
        let seed = 18;
        Self {
            genome: Genome::random(N_INPUTS, N_NODES, N_OUTPUTS, N_FUNCS, seed),
            generation: 0,
            ..Default::default()
        }
    }

    fn do_task(&self, query: String) -> TaskFuture {
        // --- synchronous logic goes here (read genome, validate, build prompt...) ---
        let provider = self.provider.clone(); // owned
        let query = query.trim().to_owned(); // example of sync preprocessing

        // --- only owned values move into the future ---
        Box::pin(async move { provider.chat(query).await })
    }
}

impl Actor for Pod {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}
    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

impl Handler<PodMessage> for Pod {
    type Result = ResponseFuture<PodResponse>;

    fn handle(&mut self, msg: PodMessage, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PodMessage::Query(query) => {
                let fut = self.do_task(query); // sync part runs now, borrow of self ends here
                Box::pin(async move {
                    match fut.await {
                        Ok(response) => PodResponse::Response(response),
                        Err(e) => PodResponse::Response(format!("Provider error: {}", e)),
                    }
                })
            }
            _ => Box::pin(async { PodResponse::Response("Pod Handler Error!".into()) }),
        }
    }
}
