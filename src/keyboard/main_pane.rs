mod request_body;
mod request_headers;
mod response_body;
mod response_data;

use crate::Application;
use crate::MainTab;

use crossterm::event::KeyEvent;

impl Application {
    pub fn handle_main_pane_input(&mut self, event: KeyEvent, main_tab: MainTab) {
        match main_tab {
            MainTab::RequestHeaders => self.handle_request_headers_input(event),
            MainTab::RequestBody => self.handle_request_body_input(event),
            MainTab::ResponseData => self.handle_response_data_input(event),
            MainTab::ResponseBody => self.handle_response_body_input(event),
        }
    }
}
