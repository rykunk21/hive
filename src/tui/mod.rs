mod actions;
mod input;
mod ui;

use crate::hive;
use actions::Action;
use crossterm::event::{self, Event};
use ratatui::{Terminal, prelude::CrosstermBackend};
use std::io;

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    hive: &mut hive::Hive,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, hive))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let Some(action) = input::handle_key(key) {
                    match action {
                        Action::Quit => return Ok(()),
                        _ => {}
                    }
                }
            }
        }
    }
}
