//! TUI module — terminal interface for monitoring and controlling the hive.
//!
//! Provides a real-time view of the pod registry, active pods, activity log,
//! and a control surface for manual hive operations.

mod actions;
mod input;
mod ui;

use crate::hive;
use actions::Action;
use crossterm::event::{self, Event, KeyCode};
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
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    hive: &mut hive::Hive,
) -> anyhow::Result<()> {
    let mut tui = Tui::new();
    loop {
        terminal.draw(|f| ui::draw(f, hive, &tui))?;
        if event::poll(std::time::Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
        {
            if let Some(action) = input::handle_key(key) {
                match action {
                    Action::Quit => return Ok(()),
                    Action::Submit => return Ok(()),
                    Action::SpawnPod => tui.state = TuiState::Spawn,
                }
            } else {
                match key.code {
                    KeyCode::Char(c) => tui.bar_input.push(c),
                    KeyCode::Backspace => {
                        tui.bar_input.pop();
                    }
                    _ => {}
                }
            }
        }
    }
}
