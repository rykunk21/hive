//! Pod actor module.
//!
//! Defines the `Pod` actor and its message protocol (`PodMessage` / `PodResponse`).
//! Pods are evolvable units that carry a `Genome` and can respond to queries
//! from the hive, replicate, or be shut down.

use actix::dev::{MessageResponse, OneshotSender};
use actix::prelude::*;

mod genome;
use genome::Genome;

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
}

impl Pod {
    pub fn new() -> Self {
        let seed = 18;
        Self {
            genome: Genome::random(N_INPUTS, N_NODES, N_OUTPUTS, N_FUNCS, seed),
            generation: 0,
        }
    }

    pub fn breed(&self, seed: u64) -> Self {
        Self {
            genome: self.genome.mutate(3, N_FUNCS, seed),
            generation: self.generation + 1,
        }
    }

    fn do_task(&self, query: String) -> String {
        todo!()
    }
}

impl Actor for Pod {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {}
    fn stopped(&mut self, _ctx: &mut Context<Self>) {}
}

impl Handler<PodMessage> for Pod {
    type Result = PodResponse;

    fn handle(&mut self, msg: PodMessage, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            PodMessage::Query(query) => {
                let res = self.do_task(query);
                PodResponse::Response(res)
            }
            _ => PodResponse::Response("No Query".into()),
        }
    }
}
