//! Hive core runtimeuse std::collections::HashMap;
use std::collections::HashMap;

// use crate::link::PodConfig;
use crate::pod::{Pod, PodMessage, PodResponse};
use actix::prelude::*;
use rand::seq::IteratorRandom;
use tokio::sync::{broadcast, mpsc};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub enum HiveCommand {
    Submit(String),
}

#[derive(Clone, Debug)]
pub enum HiveResponse {
    General(String),
    Status(HiveStatus),
    NewAgent(u8),
}

/// Track the status of the hive
#[derive(Clone, Debug)]
pub struct HiveStatus {
    agents: Vec<u8>,
}

// ---------------------------------------------------------------------------
// External interface — cloned and handed to subscribers (TUI, Discord, HTTP)
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct ExternalHandle {
    pub cmd_tx: mpsc::Sender<HiveCommand>,
    pub events_tx: broadcast::Sender<HiveResponse>, // subscribers call .subscribe()
}

// ---------------------------------------------------------------------------
// Internal interface — moved into the actix thread
// ---------------------------------------------------------------------------

pub struct InternalHandle {
    pub cmd_rx: mpsc::Receiver<HiveCommand>,
    pub events_tx: broadcast::Sender<HiveResponse>,
}

// ---------------------------------------------------------------------------
// Hive — lives on main thread, owns the external interface
// ---------------------------------------------------------------------------

pub struct Hive {
    external: ExternalHandle,
}

impl Hive {
    pub fn new() -> Self {
        let (cmd_tx, cmd_rx) = mpsc::channel::<HiveCommand>(64);
        let (events_tx, _events_rx) = broadcast::channel::<HiveResponse>(64);

        // External interface
        let external = ExternalHandle {
            cmd_tx: cmd_tx.clone(),
            events_tx: events_tx.clone(),
        };

        // Internal interface
        let internal = InternalHandle {
            cmd_rx,
            events_tx: events_tx.clone(),
        };

        std::thread::spawn(move || {
            actix::System::new().block_on(hive_loop(internal));
        });

        Hive { external }
    }

    pub fn sender(&self) -> mpsc::Sender<HiveCommand> {
        // Hive is the single consumer. We can issue a bunch of clones
        self.external.cmd_tx.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<HiveResponse> {
        // The externals can subscribe to the hive broadcast
        self.external.events_tx.subscribe()
    }

    // This function determines
}

// Default
impl Default for Hive {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn hive_loop(internal: InternalHandle) {
    let mut cmd_rx = internal.cmd_rx;
    let events_tx = internal.events_tx;
    let mut pods: HashMap<u8, Addr<Pod>> = HashMap::new();

    // Start your actor(s) inside the actix system
    loop {
        // Receive command from Hive
        match cmd_rx.recv().await {
            Some(HiveCommand::Submit(s)) => {
                // this function will add a query to a queue, to which the pods will check and
                // attempt to work on the task
                if pods.is_empty() {
                    // No pods exist, create a new one
                    let pod = Pod::new().start();
                    let id = hash(&pod);
                    pods.insert(id, pod);

                    let _ = events_tx.send(HiveResponse::NewAgent(id));
                }

                // get the id of the applicable agent
                let mut rng = rand::rng();
                let id = pods.keys().choose(&mut rng).unwrap();
                // send the query and capture the response
                let res = match pods.get(&id).unwrap().send(PodMessage::Query(s)).await {
                    Ok(PodResponse::Response(res)) => res,
                    Err(e) => {
                        format!("The pod returned an error: {}", e)
                    }
                };

                // transmit the reponse back to the Subscribers
                let _ = events_tx.send(HiveResponse::General(res));
                // let _ = events_tx.send(HiveStatus);
            }
            _ => {
                panic!("Dont know what is happening here")
            }
        }
    }
}

fn hash(pod: &Addr<Pod>) -> u8 {
    let mut hasher = DefaultHasher::new();
    pod.hash(&mut hasher);
    (hasher.finish() & 0xFF) as u8
}
