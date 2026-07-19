//! TUI module — terminal interface for monitoring and controlling the hive.
//!
//! Provides a real-time view of the pod registry, active pods, activity log,
//! and a control surface for manual hive operations.

mod input;
use crate::core::{Hive, HiveCommand, HiveResponse};

mod ui;
use ui::TuiView;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;

/// All the possible states the tui can be in
#[derive(Default)]
enum TuiState {
    #[default]
    Home,
    Spawn,
}

/// House for all the Tui specific data. Should be private, only needed by run
#[derive(Default)]
struct Tui {
    state: TuiState,
    bar_input: String,
    view: TuiView,
}

/// Run the TUI event loop.
///
/// Draws the UI at ~60 FPS (16ms frame budget), polls for keyboard input,
/// and dispatches actions to the hive. Returns when the user quits.
///
use tokio::sync::broadcast::error::TryRecvError;
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    hive: &Hive,
) -> anyhow::Result<()> {
    let mut tui = Tui::default();
    let tx = hive.sender();
    let mut rx = hive.subscribe();

    loop {
        // --- DRAIN EVENTS FROM HIVE ---
        // Non-blocking: collect all pending activity events

        // In your event drain loop:
        loop {
            match rx.try_recv() {
                Ok(event) => tui.view.activities.push(event),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Closed) => return Ok(()),
                Err(TryRecvError::Lagged(n)) => {
                    eprintln!("Lagged behind by {} events", n);
                    break;
                }
            }
        }

        // --- DRAW ---
        terminal.draw(|f| ui::draw(f, &tui))?;

        // --- HANDLE INPUT ---
        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            if let Some(action) = input::handle_key(key) {
                match action {
                    HiveCommand::Submit(_) => {
                        let text = tui.bar_input.drain(..).collect::<String>();

                        // display a copy of the input
                        tui.view
                            .activities
                            .push(HiveResponse { text: text.clone() });
                        let _ = tx.try_send(HiveCommand::Submit(text));
                    }

                    // this is going to need to be gated in input.rs
                    HiveCommand::SpawnPod(config) => {
                        let _ = tx.try_send(HiveCommand::SpawnPod(config));
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
                    (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                        // TODO: Prompt for pod type, then Some(Action::SpawnPod(type_name))
                        // TODO: Prompt for a config to spawn with
                        match tui.state {
                            TuiState::Spawn => tui.state = TuiState::Home,
                            _ => tui.state = TuiState::Spawn,
                        }
                    }
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
