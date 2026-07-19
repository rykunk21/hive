use crate::link::{PodConfig, PodMessage, PodResponse};
use crate::pods::genome::Genome;
use actix::prelude::*;
use rand::{SeedableRng, rngs::SmallRng};

const N_INPUTS: usize = 1;
const N_NODES: usize = 8;
const N_OUTPUTS: usize = 1;
const N_FUNCS: usize = 4; // expand as you define real functions

pub struct Pod {
    pub genome: Genome,
    pub generation: u32,
}

impl Pod {
    pub fn new(config: PodConfig) -> Self {
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
            PodMessage::Query(s) => {
                let active = self.genome.active_nodes().len();
                let total = self.genome.nodes.len();

                PodResponse::Response(format!(
                    "[Pod gen={} active={}/{}] {}",
                    self.generation, active, total, s
                ))
            }
            PodMessage::Spawn(config) => {
                // make a new pod? This is a bit confusing because the pod is already constructed as
                // an actor
                // thing here
                todo!();
            }
            _ => PodResponse::Response("No Query".into()),
        }
    }
}
