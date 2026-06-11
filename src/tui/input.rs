use super::actions::Action;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key(key: KeyEvent) -> Option<Action> {
    match (key.code, key.modifiers) {
        // quit
        (KeyCode::Char('q'), _) => Some(Action::Quit),
        (KeyCode::Char('c'), KeyModifiers::CONTROL) => Some(Action::Quit),

        // navigation
        /*
            (KeyCode::Down | KeyCode::Char('j'), _) => Some(Action::SelectNext),
            (KeyCode::Up | KeyCode::Char('k'), _) => Some(Action::SelectPrev),
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => Some(Action::ScrollUp),
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => Some(Action::ScrollDown),

            // pod lifecycle — placeholders, real keys TBD once UI has context
            (KeyCode::Char('s'), _) => Some(Action::SpawnPod(String::new())),
            (KeyCode::Char('x'), _) => Some(Action::KillPod(String::new())),
        */
        _ => None,
    }
}
