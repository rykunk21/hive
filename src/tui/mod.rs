//! TUI module — terminal interface for monitoring and controlling the hive.
//!
//! Provides a real-time view of the pod registry, active pods, activity log,
//! and a control surface for manual hive operations.

mod actions;
mod input;
mod ui;

use crate::hive;
use actions::Action;
use crossterm::event::{self, Event};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;

/// Run the TUI event loop.
///
/// Draws the UI at ~60 FPS (16ms frame budget), polls for keyboard input,
/// and dispatches actions to the hive. Returns when the user quits.
///
/// TODO: Replace hardcoded fake data with real hive state queries.
pub fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    hive: &mut hive::Hive,
) -> anyhow::Result<()> {
    loop {
        // TODO: Pass real hive state to draw() instead of empty Hive reference.
        terminal.draw(|f| ui::draw(f, hive))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let Some(action) = input::handle_key(key) {
                    match action {
                        Action::Quit => return Ok(()),
                        // TODO: Handle additional actions:
                        // Action::SpawnPod(type_name) => hive.spawn_pod(&type_name)?,
                        // Action::KillPod(pod_id) => hive.kill_pod(pod_id)?,
                        // Action::TriggerEvolve => hive.evolve()?,
                        // Action::SelectNext => { /* navigate UI */ },
                        // Action::SelectPrev => { /* navigate UI */ },
                    }
                }
            }
        }
    }
}
