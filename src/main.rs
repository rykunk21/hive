//! Hive TUI entry point.
//!
//! Boots the terminal UI, initializes the Hive runtime, and runs the main loop.

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use log::{debug, error, info, trace, warn};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

use hive::{core::Hive, tui};

fn main() -> Result<()> {
    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(Box::new(
            std::fs::File::create("hive.log").unwrap(),
        )))
        .init();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let hive = Hive::new();

    let result = tui::run(&mut terminal, &hive);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
