use crossterm::event::{KeyCode, KeyEvent};

use crate::{state::Method, Application};

impl Application {
    pub fn handle_method_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(character) if self.editing => {
                self.method_state.current_method = match character {
                    'g' | 'G' => Method::Get,
                    'p' | 'P' => Method::Post,
                    'o' | 'O' => Method::Options,
                    'c' | 'C' => Method::Connect,
                    't' | 'T' => Method::Trace,
                    'd' | 'D' => Method::Delete,
                    'h' | 'H' => Method::Head,
                    'u' | 'U' => Method::Put,
                    'a' | 'A' => Method::Patch,
                    _ => self.method_state.current_method.clone(),
                }
            }
            KeyCode::Enter => self.editing = !self.editing,
            KeyCode::Up => self.method_state.current_method.decrement(),
            KeyCode::Down => self.method_state.current_method.increment(),
            _ => {}
        }
    }
}
