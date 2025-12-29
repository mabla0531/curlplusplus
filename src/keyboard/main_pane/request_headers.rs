use crate::Application;
use crate::{HeaderSection, RequestHeaderFocus};

use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_request_headers_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char(character) if self.editing => {
                if let RequestHeaderFocus::Header(header) = self.main_state.current_header
                    && let Some(name) = self.main_state.headers.get_mut(header)
                {
                    match self.main_state.current_header_section {
                        HeaderSection::Name => name
                            .0
                            .insert(self.main_state.current_header_cursor, character),
                        HeaderSection::Value => name
                            .1
                            .insert(self.main_state.current_header_cursor, character),
                        _ => {}
                    }
                    self.main_state.current_header_cursor += 1;
                }
            }
            KeyCode::Backspace if self.editing => {
                if let RequestHeaderFocus::Header(header_index) = self.main_state.current_header
                    && let Some((header_name, header_value)) =
                        self.main_state.headers.get_mut(header_index)
                    && self.main_state.current_header_cursor > 0
                {
                    match self.main_state.current_header_section {
                        HeaderSection::Name => {
                            header_name
                                .remove(self.main_state.current_header_cursor.saturating_sub(1));
                        }
                        HeaderSection::Value => {
                            header_value
                                .remove(self.main_state.current_header_cursor.saturating_sub(1));
                        }
                        _ => {}
                    }

                    self.main_state.current_header_cursor =
                        self.main_state.current_header_cursor.saturating_sub(1);
                }
            }
            KeyCode::Enter => match self.main_state.current_header {
                RequestHeaderFocus::Header(index)
                    if self.main_state.current_header_section == HeaderSection::Delete =>
                {
                    self.main_state.headers.remove(index);
                    if index >= self.main_state.headers.len() {
                        self.main_state.current_header = if !self.main_state.headers.is_empty() {
                            RequestHeaderFocus::Header(
                                self.main_state.headers.len().saturating_sub(1),
                            )
                        } else {
                            RequestHeaderFocus::Add
                        }
                    }
                }
                RequestHeaderFocus::Header(header) => {
                    self.editing = !self.editing;
                    if self.editing {
                        let (name, value) = self
                            .main_state
                            .headers
                            .get(header)
                            .cloned()
                            .unwrap_or((String::new(), String::new()));

                        let section_length = match self.main_state.current_header_section {
                            HeaderSection::Name => name,
                            HeaderSection::Value => value,
                            _ => String::new(),
                        }
                        .len();

                        self.main_state.current_header_cursor = section_length;
                    }
                }
                RequestHeaderFocus::Add => {
                    self.main_state.headers.push((String::new(), String::new()))
                }
            },
            KeyCode::Up => {
                if let RequestHeaderFocus::Header(header_num) = self.main_state.current_header
                    && !self.editing
                {
                    self.main_state.current_header =
                        RequestHeaderFocus::Header(header_num.saturating_sub(1))
                } else if self.main_state.current_header == RequestHeaderFocus::Add
                    && !self.main_state.headers.is_empty()
                {
                    self.main_state.current_header =
                        RequestHeaderFocus::Header(self.main_state.headers.len().saturating_sub(1))
                }
            }
            KeyCode::Down => {
                if let RequestHeaderFocus::Header(header_num) = self.main_state.current_header
                    && !self.editing
                {
                    self.main_state.current_header =
                        if header_num >= self.main_state.headers.len().saturating_sub(1) {
                            RequestHeaderFocus::Add
                        } else {
                            RequestHeaderFocus::Header(
                                header_num
                                    .saturating_add(1)
                                    .min(self.main_state.headers.len()),
                            )
                        }
                }
            }
            KeyCode::Left if self.editing => {
                if let RequestHeaderFocus::Header(_) = self.main_state.current_header
                    && self.main_state.current_header_section != HeaderSection::Delete
                {
                    self.main_state.current_header_cursor =
                        self.main_state.current_header_cursor.saturating_sub(1);
                }
            }
            KeyCode::Left => {
                self.main_state.current_header_section.decrement();
            }

            KeyCode::Right if self.editing => match self.main_state.current_header {
                RequestHeaderFocus::Header(header) => {
                    let (name, value) = self
                        .main_state
                        .headers
                        .get(header)
                        .cloned()
                        .unwrap_or((String::new(), String::new()));
                    let section_length = match self.main_state.current_header_section {
                        HeaderSection::Name => name,
                        HeaderSection::Value => value,
                        HeaderSection::Delete => String::new(),
                    }
                    .len();

                    self.main_state.current_header_cursor =
                        (self.main_state.current_header_cursor + 1).min(section_length);
                }
                RequestHeaderFocus::Add => {}
            },
            KeyCode::Right => {
                self.main_state.current_header_section.increment();
            }

            _ => {}
        }
    }

    pub fn handle_request_headers_paste(&mut self, text: String) {}
}
