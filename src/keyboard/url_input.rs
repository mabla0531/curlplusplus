use crossterm::event::{KeyCode, KeyEvent};

use crate::{Application, defs::URL_AUTOCOMPLETES};

impl Application {
    pub fn handle_url_autocomplete(&mut self) {
        if let Some(completion) = URL_AUTOCOMPLETES
            .iter()
            .find_map(|(k, v)| self.url_state.url_input.eq(k).then_some(*v))
        {
            self.url_state.url_input.push_str(completion);
            self.url_state.url_cursor = self.url_state.url_input.len();
        }
    }

    pub fn handle_url_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(character) if self.editing => {
                self.url_state
                    .url_input
                    .insert(self.url_state.url_cursor, character);
                self.url_state.url_cursor =
                    (self.url_state.url_cursor + 1).min(self.url_state.url_input.len());
            }
            KeyCode::Tab if self.editing => {
                self.handle_url_autocomplete();
            }
            KeyCode::Left if self.editing => {
                self.url_state.url_cursor = self.url_state.url_cursor.saturating_sub(1)
            }
            KeyCode::Right if self.editing => {
                self.url_state.url_cursor =
                    (self.url_state.url_cursor + 1).min(self.url_state.url_input.len())
            }
            KeyCode::Enter => self.editing = !self.editing,
            KeyCode::Backspace if self.editing && self.url_state.url_cursor > 0 => {
                self.url_state
                    .url_input
                    .remove(self.url_state.url_cursor.saturating_sub(1));
                self.url_state.url_cursor = self.url_state.url_cursor.saturating_sub(1);
            }
            _ => {}
        }
    }
}
