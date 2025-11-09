use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    Application,
    state::{Panel, RequestTab, ResponseTab},
};

use crate::state::Method;

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(character) => match self.state.focused_panel {
                Panel::Method => {
                    self.state.current_method = match character {
                        'g' | 'G' => Method::Get,
                        'p' | 'P' => Method::Post,
                        'o' | 'O' => Method::Options,
                        'c' | 'C' => Method::Connect,
                        't' | 'T' => Method::Trace,
                        'd' | 'D' => Method::Delete,
                        'h' | 'H' => Method::Head,
                        'u' | 'U' => Method::Put,
                        'a' | 'A' => Method::Patch,
                        _ => self.state.current_method.clone(),
                    }
                }
                Panel::Url => self.state.url_input.push(character),
                _ => {}
            },
            KeyCode::BackTab => {
                self.state.focused_panel = match self.state.focused_panel {
                    Panel::Method => Panel::Response,
                    Panel::Url => Panel::Method,
                    Panel::Request => Panel::Url,
                    Panel::Response => Panel::Request,
                }
            }
            KeyCode::Backspace if self.state.focused_panel == Panel::Url => {
                self.state.url_input.pop();
            }
            KeyCode::Tab => {
                self.state.show_method_dropdown = false;
                self.state.focused_panel = match self.state.focused_panel {
                    Panel::Method => Panel::Url,
                    Panel::Url => Panel::Request,
                    Panel::Request => Panel::Response,
                    Panel::Response => Panel::Method,
                }
            }
            KeyCode::Enter => match self.state.focused_panel {
                Panel::Method => self.state.show_method_dropdown = !self.state.show_method_dropdown,
                Panel::Url => {}
                Panel::Request => {}
                Panel::Response => {}
            },
            KeyCode::Up => match self.state.focused_panel {
                Panel::Method => {
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

                _ => {}
            },
            KeyCode::Down => match self.state.focused_panel {
                Panel::Method => {
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
                _ => {}
            },
            KeyCode::Left => match self.state.focused_panel {
                Panel::Request => {
                    self.state.current_request_tab = match self.state.current_request_tab {
                        RequestTab::Headers => RequestTab::Settings,
                        RequestTab::Body => RequestTab::Headers,
                        RequestTab::Settings => RequestTab::Body,
                    }
                }
                Panel::Response => {
                    self.state.current_response_tab = match self.state.current_response_tab {
                        ResponseTab::Data => ResponseTab::Body,
                        ResponseTab::Body => ResponseTab::Data,
                    }
                }
                _ => {}
            },
            KeyCode::Right => match self.state.focused_panel {
                Panel::Url => {
                    if self.state.url_input == "https" || self.state.url_input == "http" {
                        self.state.url_input.push_str("://");
                    }
                    if "htt".starts_with(self.state.url_input.as_str())
                        || self.state.url_input.is_empty()
                    {
                        self.state.url_input = "http".to_string();
                    }
                    if self.state.url_input.ends_with(".") {
                        self.state.url_input.push_str("com");
                    }
                }
                Panel::Request => {
                    self.state.current_request_tab = match self.state.current_request_tab {
                        RequestTab::Headers => RequestTab::Body,
                        RequestTab::Body => RequestTab::Settings,
                        RequestTab::Settings => RequestTab::Headers,
                    }
                }
                Panel::Response => {
                    self.state.current_response_tab = match self.state.current_response_tab {
                        ResponseTab::Data => ResponseTab::Body,
                        ResponseTab::Body => ResponseTab::Data,
                    }
                }
                _ => {}
            },
            KeyCode::Esc => self.exit_request = true,
            _ => {}
        }
    }
}
