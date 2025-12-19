use crossterm::event::{KeyCode, KeyEvent};

use crate::Application;

impl Application {
    pub fn handle_url_autocomplete(&mut self) {
        if self.url_state.url_input == "https" || self.url_state.url_input == "http" {
            self.url_state.url_input.push_str("://");
            self.url_state.url_cursor = self.url_state.url_input.len();
        }
        if "htt".starts_with(self.url_state.url_input.as_str())
            || self.url_state.url_input.is_empty()
        {
            self.url_state.url_input = "http".to_string();
            self.url_state.url_cursor = self.url_state.url_input.len();
        }
        if self.url_state.url_input.ends_with(".") {
            self.url_state.url_input.push_str("com");
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
