mod method;
mod main_pane;
mod url_input;

use crossterm::event::{KeyCode, KeyEvent};

use crate::{Application, state::Panel};

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::BackTab => {
                if self.editing {
                    match self.focused_panel {
                        Panel::Method => self.handle_method_input(event),
                        Panel::Url => self.handle_url_input(event),
                        Panel::Main(main_tab) => {
                            self.handle_main_pane_input(event, main_tab)
                        }
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
                        Panel::Main(main_tab) => {
                            self.handle_main_pane_input(event, main_tab)
                        }
                    }
                } else {
                    self.method_state.show_dropdown = false;
                    self.focused_panel.increment();
                }
            }
            KeyCode::Enter => match self.focused_panel {
                Panel::Method => self.handle_method_input(event),
                Panel::Url => self.handle_url_input(event),
                Panel::Main(main_tab) => self.handle_main_pane_input(event, main_tab),
            },
            KeyCode::Esc => {
                if self.editing {
                    self.editing = false;
                } else {
                    self.exit_request = true;
                }
            }
            _ => match self.focused_panel {
                Panel::Method => self.handle_method_input(event),
                Panel::Url => self.handle_url_input(event),
                Panel::Main(main_tab) => self.handle_main_pane_input(event, main_tab),
            },
        }
    }
}
