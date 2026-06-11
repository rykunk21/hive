//! Keyboard input handling — maps keystrokes to TUI actions.
//!
//! Supports vim-style navigation and single-key commands for pod lifecycle
/// management. All real actions are stubbed until hive implements them.

use super::actions::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Convert a keyboard event into a TUI action.
///
/// Returns `None` for unbound keys. The TUI event loop ignores unmapped input.
///
/// TODO: Wire navigation and pod lifecycle keys once hive supports the actions.
pub fn handle_key(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        // Quit
        (KeyCode::Char('q'), _) => Some(Action::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Action::Quit),

        // Navigation — stubbed until UI has focusable panels
        /*
        (KeyCode::Down | KeyCode::Char('j'), _) => Some(Action::SelectNext),
        (KeyCode::Up | KeyCode::Char('k'), _) => Some(Action::SelectPrev),
        (KeyCode::Char('u'), KeyModifiers::CONTROL) => Some(Action::ScrollUp),
        (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Action::ScrollDown),
        */

        // Pod lifecycle — stubbed until Hive::spawn_pod / Hive::kill_pod exist
        /*
        (KeyCode::Char('s'), _) => {
            // TODO: Prompt for pod type, then Some(Action::SpawnPod(type_name))
            None
        }
        (KeyCode::Char('x'), _) => {
            // TODO: Prompt for pod ID, then Some(Action::KillPod(pod_id))
            None
        }
        */

        // Self-modification trigger — stubbed until Hive::evolve exists
        /*
        (KeyCode::Char('e'), _) => Some(Action::TriggerEvolve),
        */

        _ => None,
    }
}
