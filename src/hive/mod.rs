//! Hive core runtime.

use crate::pod::PodActor;
use crate::pod::{Task, TaskResult};
use crate::pods::speaker::SpeakerPod;
use actix::prelude::*;
use std::sync::mpsc;
use std::thread;

// ---------------------------------------------------------------------------
// Activity events — what the TUI reads back
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ActivityEvent {
    pub text: String,
}

// ---------------------------------------------------------------------------
// Internal command — what the shell sends to the inner loop
// ---------------------------------------------------------------------------

enum HiveCommand {
    Submit(String),
}

// ---------------------------------------------------------------------------
// Hive shell — lives on the main thread, owned by the TUI
// ---------------------------------------------------------------------------

pub struct Hive {
    tx_cmd: mpsc::Sender<HiveCommand>,
    rx_events: mpsc::Receiver<ActivityEvent>,
}

impl Hive {
    pub fn new() -> Self {
        let (tx_cmd, rx_cmd) = mpsc::channel::<HiveCommand>();
        let (tx_events, rx_events) = mpsc::channel::<ActivityEvent>();

        // Spin up actix on its own thread
        thread::spawn(move || {
            actix::System::new().block_on(hive_loop(rx_cmd, tx_events));
        });

        Hive { tx_cmd, rx_events }
    }

    /// Submit text from the TUI input bar.
    pub fn submit(&self, text: String) {
        let _ = self.tx_cmd.send(HiveCommand::Submit(text));
    }

    /// Drain any new activity events — call each frame from the TUI.
    pub fn drain_events(&self) -> Vec<ActivityEvent> {
        self.rx_events.try_iter().collect()
    }
}

// ---------------------------------------------------------------------------
// Inner async loop — lives on the actix thread
// ---------------------------------------------------------------------------

async fn hive_loop(rx_cmd: mpsc::Receiver<HiveCommand>, tx_events: mpsc::Sender<ActivityEvent>) {
    // Spawn the speaker pod actor
    let addr: Addr<PodActor<SpeakerPod>> = PodActor::start_for(SpeakerPod::new());

    tx_events
        .send(ActivityEvent {
            text: "hive started".into(),
        })
        .ok();
    tx_events
        .send(ActivityEvent {
            text: "speaker pod spawned".into(),
        })
        .ok();

    loop {
        // Poll the command channel — try_recv is non-blocking
        match rx_cmd.try_recv() {
            Ok(HiveCommand::Submit(text)) => {
                tx_events
                    .send(ActivityEvent {
                        text: format!("› {}", text),
                    })
                    .ok();

                let task = Task::text(text);
                let tx = tx_events.clone();

                // Send to actor — addr.send() returns a future
                let fut = addr.send(task);
                actix::spawn(async move {
                    match fut.await {
                        Ok(Ok(result)) => {
                            tx.send(ActivityEvent {
                                text: format!("◄ {}", result.response),
                            })
                            .ok();
                        }
                        Ok(Err(e)) => {
                            tx.send(ActivityEvent {
                                text: format!("✖ error: {}", e),
                            })
                            .ok();
                        }
                        Err(e) => {
                            tx.send(ActivityEvent {
                                text: format!("✖ mailbox error: {}", e),
                            })
                            .ok();
                        }
                    }
                });
            }
            Err(mpsc::TryRecvError::Disconnected) => break,
            Err(mpsc::TryRecvError::Empty) => {}
        }

        // Yield to actix so actor futures can make progress
        tokio::time::sleep(std::time::Duration::from_millis(16)).await;
    }
}

