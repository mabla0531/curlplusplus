use crossterm::event::{KeyCode, KeyEvent};

use crate::{Application, state::FocusedPanel};

use crate::state::Method;

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(char) => match char {
                'q' | 'Q' => self.exit_request = true,
                _ => {}
            },
            KeyCode::BackTab => {
                self.state.focused_panel = match self.state.focused_panel {
                    FocusedPanel::Method => FocusedPanel::Response,
                    FocusedPanel::Url => FocusedPanel::Method,
                    FocusedPanel::Request => FocusedPanel::Url,
                    FocusedPanel::Response => FocusedPanel::Request,
                }
            }
            KeyCode::Tab => {
                self.state.focused_panel = match self.state.focused_panel {
                    FocusedPanel::Method => FocusedPanel::Url,
                    FocusedPanel::Url => FocusedPanel::Request,
                    FocusedPanel::Request => FocusedPanel::Response,
                    FocusedPanel::Response => FocusedPanel::Method,
                }
            }
            KeyCode::Enter => match self.state.focused_panel {
                FocusedPanel::Method => {
                    self.state.show_method_dropdown = !self.state.show_method_dropdown
                }
                FocusedPanel::Url => {}
                FocusedPanel::Request => {}
                FocusedPanel::Response => {}
            },
            KeyCode::Up => match self.state.focused_panel {
                FocusedPanel::Method => {
                    if self.state.show_method_dropdown {
                        self.state.current_method = match self.state.current_method {
                            Method::Get => Method::Head,
                            Method::Post => Method::Get,
                            Method::Put => Method::Post,
                            Method::Patch => Method::Put,
                            Method::Options => Method::Patch,
                            Method::Connect => Method::Options,
                            Method::Trace => Method::Connect,
                            Method::Delete => Method::Trace,
                            Method::Head => Method::Delete,
                        }
                    }
                }

                _ => {}
            },
            KeyCode::Down => match self.state.focused_panel {
                FocusedPanel::Method => {
                    if self.state.show_method_dropdown {
                        self.state.current_method = match self.state.current_method {
                            Method::Get => Method::Post,
                            Method::Post => Method::Put,
                            Method::Put => Method::Patch,
                            Method::Patch => Method::Options,
                            Method::Options => Method::Connect,
                            Method::Connect => Method::Trace,
                            Method::Trace => Method::Delete,
                            Method::Delete => Method::Head,
                            Method::Head => Method::Get,
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}
