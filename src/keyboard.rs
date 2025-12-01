mod method;
mod request_pane;
mod url_input;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{Application, state::Panel};

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::BackTab => {
                self.method_state.show_dropdown = false;
                self.focused_panel.decrement();
            }
            KeyCode::Tab => {
                self.method_state.show_dropdown = false;
                self.focused_panel.increment();
            }
            KeyCode::Esc => self.exit_request = true,
            _ => match self.focused_panel {
                Panel::Method => self.handle_method_input(event),
                Panel::Url => self.handle_url_input(event),
                Panel::Request(request_tab) => self.handle_request_pane_input(event, request_tab),
                _ => {}
            },
        }
    }
}
