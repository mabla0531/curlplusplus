use crossterm::event::KeyCode;

use crate::{Application, state::FocusedPanel};

impl Application {
    pub fn handle_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(char) => match char {
                'q' => self.exit_request = true,
                _ => {}
            },
            KeyCode::Tab => {
                self.state.focused_panel = match self.state.focused_panel {
                    FocusedPanel::Method => FocusedPanel::Url,
                    FocusedPanel::Url => FocusedPanel::Request,
                    FocusedPanel::Request => FocusedPanel::Response,
                    FocusedPanel::Response => FocusedPanel::Method,
                }
            }
            _ => {}
        }
    }
}
