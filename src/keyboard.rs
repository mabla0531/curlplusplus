use crossterm::event::{KeyCode, KeyEvent};

use crate::{Application, state::FocusedPanel};

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(char) => match char {
                'q' | 'Q' => self.exit_request = true,
                _ => {}
            },
            KeyCode::BackTab => {
                self.state.focused_panel = match self.state.focused_panel {
                    FocusedPanel::Method { .. } => FocusedPanel::Response,
                    FocusedPanel::Url => FocusedPanel::Method {
                        show_dropdown: false,
                    },
                    FocusedPanel::Request => FocusedPanel::Url,
                    FocusedPanel::Response => FocusedPanel::Request,
                }
            }
            KeyCode::Tab => {
                self.state.focused_panel = match self.state.focused_panel {
                    FocusedPanel::Method { .. } => FocusedPanel::Url,
                    FocusedPanel::Url => FocusedPanel::Request,
                    FocusedPanel::Request => FocusedPanel::Response,
                    FocusedPanel::Response => FocusedPanel::Method {
                        show_dropdown: false,
                    },
                }
            }
            KeyCode::Enter => match self.state.focused_panel {
                FocusedPanel::Method { mut show_dropdown } => show_dropdown = !show_dropdown,
                FocusedPanel::Url => {}
                FocusedPanel::Request => {}
                FocusedPanel::Response => {}
            },
            _ => {}
        }
    }
}
