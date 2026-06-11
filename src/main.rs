//! Hive TUI entry point.
//!
//! Boots the terminal UI, initializes the Hive runtime from configuration,
//! and runs the main event loop. On shutdown, restores terminal state.

use std::io;

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

mod config;
mod hive;
mod tui;

fn main() -> Result<()> {
    // TODO: Load hive.toml configuration before initializing terminal.
    // TODO: Initialize Hive::new(config) with real config instead of empty default.

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut hive = hive::Hive::new();

    let result = tui::run(&mut terminal, &mut hive);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // TODO: Persist hive state and knowledge base before exiting.
    result
}
