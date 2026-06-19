//! Hive core runtimeuse std::collections::HashMap;
use crate::pods::{
    ping::{Ping, PingMessages, PingResponses},
    speaker::{Speaker, SpeakerMessage, SpeakerResponse},
};
use actix::prelude::*;
use tokio::sync::{broadcast, mpsc};

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub enum HiveCommand {
    Ping,
    Submit(String),
    SpawnPod,
}

#[derive(Clone, Debug)]
pub struct HiveResponse {
    pub text: String,
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

        let external = ExternalHandle {
            cmd_tx: cmd_tx.clone(),
            events_tx: events_tx.clone(),
        };

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
        self.external.cmd_tx.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<HiveResponse> {
        self.external.events_tx.subscribe()
    }
}

// Default
impl Default for Hive {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Hive event loop — runs inside actix on a background thread
// ---------------------------------------------------------------------------

async fn hive_loop(internal: InternalHandle) {
    let mut cmd_rx = internal.cmd_rx;
    let events_tx = internal.events_tx;

    // Start your actor(s) inside the actix system
    let ping = Ping.start();
    let speak = Speaker::new().start();

    loop {
        // Receive command from Hive
        match cmd_rx.recv().await {
            Some(HiveCommand::Ping) => {
                let ping_fut = ping.send(PingMessages::Ping);
                let pong_fut = ping.send(PingMessages::Pong);

                let (ping_res, pong_res) = futures::join!(ping_fut, pong_fut);

                for (label, res) in [("Ping", ping_res), ("Pong", pong_res)] {
                    let text = match res {
                        Ok(PingResponses::GotPing) => format!("{}: GotPing", label),
                        Ok(PingResponses::GotPong) => format!("{}: GotPong", label),
                        Err(e) => format!("{} error: {}", label, e),
                    };
                    let _ = events_tx.send(HiveResponse { text });
                }
            }
            Some(HiveCommand::SpawnPod) => {
                // Handle spawn...
            }
            Some(HiveCommand::Submit(text)) => {
                let res = speak.send(SpeakerMessage::Prompt(text.clone())).await;
                match res {
                    Ok(SpeakerResponse::Response(r)) => {
                        let _ = events_tx.send(HiveResponse { text: r });
                    }
                    Err(mailbox_err) => {
                        panic!("Speaker Response Err: {}", mailbox_err)
                    }
                }
            }
            None => {
                // All senders dropped, channel closed — shut down
                break;
            }
        }
    }
}
