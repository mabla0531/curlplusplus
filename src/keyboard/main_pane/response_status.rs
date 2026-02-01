use crate::{Application, client::ResponseType, ui::response_status_height};
use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_response_status_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Up => {
                self.main_state.response_status_scroll =
                    self.main_state.response_status_scroll.saturating_sub(1);
            }

            KeyCode::Down => {
                let last_response = self.last_response.lock().unwrap();
                let last_response = last_response.as_ref();

                if let ResponseType::FinishedSuccess(response) = last_response {
                    let total_response_height =
                        response_status_height(self.terminal_width, response);

                    self.main_state.response_status_scroll =
                        (self.main_state.response_status_scroll + 1).min(
                            total_response_height
                                .saturating_sub(self.response_status_pane_height() as usize),
                        );
                }
            }
            _ => {}
        }
    }
}
