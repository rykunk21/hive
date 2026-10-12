//! Keyboard input handling — maps keystrokes to TUI actions.
//!
//! Supports vim-style navigation and single-key commands for pod lifecycle
use hive::core::HiveCommand;

/// management. All real actions are stubbed until hive implements them.
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Convert a keyboard event into a TUI action.
///
/// Returns `None` for unbound keys. The TUI event loop ignores unmapped input.
///
/// TODO: Wire navigation and pod lifecycle keys once hive supports the actions.
pub fn handle_key(key: KeyEvent) -> Option<HiveCommand> {
    match (key.code, key.modifiers) {
        // Ping
        (KeyCode::Char('p'), KeyModifiers::CONTROL) => Some(HiveCommand::Ping),

        // Submit input
        (KeyCode::Enter, _) => Some(HiveCommand::Submit("".into())),

        // Navigation — stubbed until UI has focusable panels
        /*
        (KeyCode::Down | KeyCode::Char('j'), _) => Some(Action::SelectNext),
        (KeyCode::Up | KeyCode::Char('k'), _) => Some(Action::SelectPrev),
        */
        // Pod lifecycle — stubbed until Hive::spawn_pod / Hive::kill_pod exist
        (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
            // TODO: Prompt for pod type, then Some(Action::SpawnPod(type_name))
            Some(HiveCommand::SpawnPod)
        }

        _ => None,
    }
}
