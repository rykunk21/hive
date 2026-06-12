//! TUI module — terminal interface for monitoring and controlling the hive.
//!
//! Provides a real-time view of the pod registry, active pods, activity log,
//! and a control surface for manual hive operations.

mod input;
mod ui;

use crate::hive::{self, HiveCommand, HiveResponse};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;

/// All the possible states the tui can be in
enum TuiState {
    Home,
    Spawn,
}

/// House for all the Tui specific data. Should be private, only needed by run
struct Tui {
    state: TuiState,
    bar_input: String,
}

impl Tui {
    fn new() -> Self {
        Tui {
            state: TuiState::Home,
            bar_input: String::new(),
        }
    }
}

/// Run the TUI event loop.
///
/// Draws the UI at ~60 FPS (16ms frame budget), polls for keyboard input,
/// and dispatches actions to the hive. Returns when the user quits.
///
use tokio::sync::broadcast::error::TryRecvError;
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    hive: &hive::Hive,
) -> anyhow::Result<()> {
    let mut tui = Tui::new();
    let tx = hive.sender();
    let mut rx = hive.subscribe();
    let mut activity = vec![];

    loop {
        // --- DRAIN EVENTS FROM HIVE ---
        // Non-blocking: collect all pending activity events

        // In your event drain loop:
        loop {
            match rx.try_recv() {
                Ok(event) => activity.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => return Ok(()),
                Err(TryRecvError::Lagged(n)) => {
                    eprintln!("Lagged behind by {} events", n);
                    break;
                }
            }
        }

        // --- DRAW ---
        terminal.draw(|f| ui::draw(f, &tui, &activity))?;

        // --- HANDLE INPUT ---
        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            if let Some(action) = input::handle_key(key) {
                match action {
                    HiveCommand::Submit(_) => {
                        let text = tui.bar_input.drain(..).collect::<String>();

                        // display a copy of the input
                        activity.push(HiveResponse { text: text.clone() });
                        let _ = tx.try_send(HiveCommand::Submit(text));
                    }

                    HiveCommand::SpawnPod => {
                        tui.state = match tui.state {
                            TuiState::Spawn => TuiState::Home,
                            _ => TuiState::Spawn,
                        }
                    }
                    HiveCommand::Ping => {
                        let _ = tx.try_send(HiveCommand::Ping);
                    }
                }
            } else {
                match (key.code, key.modifiers) {
                    // Quit
                    (KeyCode::Char('q'), _) => return Ok(()),
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(()),

                    (KeyCode::Char(c), _) => tui.bar_input.push(c),
                    (KeyCode::Backspace, _) => {
                        tui.bar_input.pop();
                    }
                    _ => {}
                }
            }
        }
    }
}
