mod request_body;
mod request_headers;
mod response_status;

use crate::{Application, state::MainTab};

use crossterm::event::KeyEvent;

impl Application {
    pub fn handle_main_pane_input(&mut self, event: KeyEvent, main_tab: MainTab) {
        match main_tab {
            MainTab::RequestHeaders => self.handle_request_headers_input(event),
            MainTab::RequestBody => self.handle_request_body_input(event),
            MainTab::ResponseStatus => self.handle_response_status_input(event),
        }
    }
}
