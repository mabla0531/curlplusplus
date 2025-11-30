use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    Application,
    state::{BodyCursor, HeaderSection, Panel, RequestHeaderFocus, RequestTab},
};

use crate::state::Method;

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(character) => match self.focused_panel {
                Panel::Method => {
                    self.method_state.current_method = match character {
                        'g' | 'G' => Method::Get,
                        'p' | 'P' => Method::Post,
                        'o' | 'O' => Method::Options,
                        'c' | 'C' => Method::Connect,
                        't' | 'T' => Method::Trace,
                        'd' | 'D' => Method::Delete,
                        'h' | 'H' => Method::Head,
                        'u' | 'U' => Method::Put,
                        'a' | 'A' => Method::Patch,
                        _ => self.method_state.current_method.clone(),
                    }
                }
                Panel::Url => self.url_state.url_input.push(character),
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => {
                        if let RequestHeaderFocus::Header(header) =
                            self.request_state.current_header
                            && let Some(name) = self.request_state.headers.get_mut(header)
                        {
                            match self.request_state.current_header_section {
                                HeaderSection::Name => name.0.push(character),
                                HeaderSection::Value => name.1.push(character),
                                HeaderSection::Delete => {}
                            }
                        }
                    }
                    RequestTab::Body => {
                        let body_cursor: &mut BodyCursor = &mut self.request_state.body_cursor;
                        if let Some(line) = self.request_state.body.get_mut(body_cursor.line) {
                            line.insert(body_cursor.column.min(line.len()), character);
                            body_cursor.column =
                                (body_cursor.column + 1).min(line.len().saturating_sub(1));
                        }
                    }
                },
                _ => {}
            },
            KeyCode::BackTab => {
                self.method_state.show_dropdown = false;
                self.focused_panel.decrement();
            }
            KeyCode::Tab => {
                self.method_state.show_dropdown = false;
                self.focused_panel.increment();
            }
            KeyCode::Backspace => match self.focused_panel {
                Panel::Url => {
                    self.url_state.url_input.pop();
                }
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => {
                        if let RequestHeaderFocus::Header(header_index) =
                            self.request_state.current_header
                            && let Some((header_name, header_value)) =
                                self.request_state.headers.get_mut(header_index)
                        {
                            match self.request_state.current_header_section {
                                HeaderSection::Name => {
                                    header_name.pop();
                                }
                                HeaderSection::Value => {
                                    header_value.pop();
                                }
                                _ => {}
                            }
                        }
                    }
                    RequestTab::Body => {
                        let body_cursor: &mut BodyCursor = &mut self.request_state.body_cursor;
                        let body = &mut self.request_state.body;
                        if let Some(line) = body.get_mut(body_cursor.line) {
                            if line.is_empty() {
                                body.remove(body_cursor.line);
                                body_cursor.line = body_cursor.line.saturating_sub(1);
                                body_cursor.column = self
                                    .request_state
                                    .body
                                    .get(body_cursor.line)
                                    .unwrap_or(&String::new())
                                    .len()
                                    .saturating_sub(1);
                            } else {
                                if body_cursor.column < line.len() - 1 {
                                    line.remove(body_cursor.column);
                                } else {
                                    line.pop();
                                }
                                body_cursor.column = body_cursor.column.saturating_sub(1);
                            }
                        } else {
                            body_cursor.line = body.len().saturating_sub(1);
                        }
                    }
                },
                _ => {}
            },
            KeyCode::Enter => match &self.focused_panel {
                Panel::Method => self.method_state.show_dropdown = !self.method_state.show_dropdown,
                Panel::Url => {}
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => match self.request_state.current_header {
                        RequestHeaderFocus::Header(index) => {
                            if self.request_state.current_header_section == HeaderSection::Delete {
                                self.request_state.headers.remove(index);
                                if index >= self.request_state.headers.len() {
                                    self.request_state.current_header =
                                        if !self.request_state.headers.is_empty() {
                                            RequestHeaderFocus::Header(
                                                self.request_state.headers.len() - 1,
                                            )
                                        } else {
                                            RequestHeaderFocus::Add
                                        }
                                }
                            }
                        }
                        RequestHeaderFocus::Add => self
                            .request_state
                            .headers
                            .push((String::new(), String::new())),
                    },
                    RequestTab::Body => {
                        self.request_state.body.push(String::new());
                        self.request_state.body_cursor.line += 1;
                        self.request_state.body_cursor.column = 0;
                    }
                },
                Panel::Response(response_tab) => {}
            },
            KeyCode::Up => match &self.focused_panel {
                Panel::Method => self.method_state.current_method.decrement(),
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => {
                        self.request_state.current_header = match self.request_state.current_header
                        {
                            RequestHeaderFocus::Header(header_num) => {
                                RequestHeaderFocus::Header(header_num.saturating_sub(1))
                            }
                            RequestHeaderFocus::Add => {
                                if !self.request_state.headers.is_empty() {
                                    RequestHeaderFocus::Header(
                                        self.request_state.headers.len().saturating_sub(1),
                                    )
                                } else {
                                    RequestHeaderFocus::Add
                                }
                            }
                        }
                    }
                    RequestTab::Body => {
                        let body_cursor = &mut self.request_state.body_cursor;

                        body_cursor.line = body_cursor.line.saturating_sub(1);
                    }
                },

                _ => {}
            },
            KeyCode::Down => match &self.focused_panel {
                Panel::Method => self.method_state.current_method.increment(),
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => {
                        if let RequestHeaderFocus::Header(header_num) =
                            self.request_state.current_header
                        {
                            self.request_state.current_header =
                                if header_num >= self.request_state.headers.len() - 1 {
                                    RequestHeaderFocus::Add
                                } else {
                                    RequestHeaderFocus::Header(
                                        header_num
                                            .saturating_add(1)
                                            .min(self.request_state.headers.len()),
                                    )
                                }
                        }
                    }
                    RequestTab::Body => {
                        let body_cursor = &mut self.request_state.body_cursor;

                        body_cursor.line =
                            (body_cursor.line + 1).min(self.request_state.body.len() - 1);
                    }
                },
                _ => {}
            },
            KeyCode::Left => match &self.focused_panel {
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => {
                        self.request_state.current_header_section.decrement();
                    }
                    RequestTab::Body => {
                        let body_cursor = &mut self.request_state.body_cursor;
                        if body_cursor.column < 1 {
                            body_cursor.line = body_cursor.line.saturating_sub(1);
                            body_cursor.column = self
                                .request_state
                                .body
                                .get(body_cursor.line)
                                .unwrap_or(&String::new())
                                .len()
                                .saturating_sub(1);
                        } else {
                            body_cursor.column = body_cursor.column.saturating_sub(1);
                        }
                    }
                },
                Panel::Response(response_tab) => {}
                _ => {}
            },
            KeyCode::Right => match &self.focused_panel {
                Panel::Url => {
                    if self.url_state.url_input == "https" || self.url_state.url_input == "http" {
                        self.url_state.url_input.push_str("://");
                    }
                    if "htt".starts_with(self.url_state.url_input.as_str())
                        || self.url_state.url_input.is_empty()
                    {
                        self.url_state.url_input = "http".to_string();
                    }
                    if self.url_state.url_input.ends_with(".") {
                        self.url_state.url_input.push_str("com");
                    }
                }
                Panel::Request(request_tab) => match request_tab {
                    RequestTab::Headers => {
                        self.request_state.current_header_section.increment();
                    }
                    RequestTab::Body => {
                        let body_cursor = &mut self.request_state.body_cursor;
                        if body_cursor.column
                            >= self
                                .request_state
                                .body
                                .get(body_cursor.line)
                                .unwrap_or(&String::new())
                                .len()
                                .saturating_sub(1)
                        {
                            body_cursor.line = (body_cursor.line + 1)
                                .min(self.request_state.body.len().saturating_sub(1));
                            body_cursor.column = self
                                .request_state
                                .body
                                .get(body_cursor.line)
                                .unwrap_or(&String::new())
                                .len()
                                .saturating_sub(1);
                        } else {
                            body_cursor.column += 1;
                        }
                    }
                },
                Panel::Response(response_tab) => {}
                _ => {}
            },
            KeyCode::Esc => self.exit_request = true,
            _ => {}
        }
    }
}
