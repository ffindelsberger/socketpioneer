use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::AppState;

pub fn update(state: &mut AppState, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => state.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                state.quit()
            }
        }
        KeyCode::Right | KeyCode::Char('j') => state.increment_counter(),
        KeyCode::Left | KeyCode::Char('k') => state.decrement_counter(),
        _ => {}
    };
}
