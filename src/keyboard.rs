mod method;
mod url_input;
mod request_pane;
mod response_pane;

use crossterm::{execute, cursor::SetCursorStyle, event::{KeyCode, KeyEvent}};

use crate::{Application, state::Panel};

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::BackTab => {
                if self.editing {
                    match self.focused_panel {
                        Panel::Method => self.handle_method_input(event),
                        Panel::Url => self.handle_url_input(event),
                        Panel::Request(request_tab) => self.handle_request_pane_input(event, request_tab),
                        Panel::Response(response_tab) => self.handle_response_pane_input(event, response_tab),
                    } 
                } else {
                    self.method_state.show_dropdown = false;
                    self.focused_panel.decrement();
                }
            }
            KeyCode::Tab => {
                if self.editing {
                    match self.focused_panel {
                        Panel::Method => self.handle_method_input(event),
                        Panel::Url => self.handle_url_input(event),
                        Panel::Request(request_tab) => self.handle_request_pane_input(event, request_tab),
                        Panel::Response(response_tab) => self.handle_response_pane_input(event, response_tab),
                    }
                } else {
                    self.method_state.show_dropdown = false;
                    self.focused_panel.increment();
                }
            }
            KeyCode::Enter => {
                match self.focused_panel {
                    Panel::Method => self.handle_method_input(event),
                    Panel::Url => self.handle_url_input(event),
                    Panel::Request(request_tab) => self.handle_request_pane_input(event, request_tab),
                    Panel::Response(response_tab) => self.handle_response_pane_input(event, response_tab),
                }
            }
            KeyCode::Esc => {
                if self.editing {
                    self.editing = false;
                } else {
                    self.exit_request = true;
                }
            },
            _ => match self.focused_panel {
                Panel::Method => self.handle_method_input(event),
                Panel::Url => self.handle_url_input(event),
                Panel::Request(request_tab) => self.handle_request_pane_input(event, request_tab),
                Panel::Response(response_tab) => self.handle_response_pane_input(event, response_tab),
            },
        }
    }
}

