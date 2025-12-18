use crate::Application;
use crate::state::ResponseTab;
use crossterm::event::KeyEvent;

impl Application {
    pub fn handle_response_pane_input(&mut self, event: KeyEvent, response_tab: ResponseTab) {
        match event.code {
            _ => {}
        }
    }
}
