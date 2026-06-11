//! Hive TUI entry point.
//!
//! Boots the terminal UI, initializes the Hive runtime, and runs the main loop.

use std::io;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod config;
mod hive;
mod pod;
mod pods;
mod tui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut hive = hive::Hive::new();

    // TODO: Register built-in pod types with hive.registry
    // hive.registry.register("speaker", Box::new(|| Box::new(pods::speaker::SpeakerPod::default())));

    let result = tui::run(&mut terminal, &mut hive);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
