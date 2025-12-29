use crate::Application;
use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_response_body_input(&mut self, event: KeyEvent) {}

    pub fn handle_response_body_paste(&mut self, text: String) {}
}
