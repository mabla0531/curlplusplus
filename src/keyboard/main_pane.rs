use crate::Application;
use crate::{BodyCursor, HeaderSection, RequestHeaderFocus, MainTab};

use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_main_pane_input(&mut self, event: KeyEvent, main_tab: MainTab) {
        match event.code {
            KeyCode::Char(character) => match main_tab {
                MainTab::RequestHeaders if self.editing => {
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
                MainTab::RequestBody if self.editing => {
                    let body_cursor: &mut BodyCursor = &mut self.main_state.request_body_cursor;
                    if let Some(line) = self.main_state.request_body.get_mut(body_cursor.line) {
                        line.insert(body_cursor.column.min(line.len()), character);
                        body_cursor.column = (body_cursor.column + 1).min(line.len());
                        match character {
                            '{' => line.insert(body_cursor.column.min(line.len()), '}'),
                            '[' => line.insert(body_cursor.column.min(line.len()), ']'),
                            '\"' => line.insert(body_cursor.column.min(line.len()), '\"'),
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Backspace if self.editing => match main_tab {
                MainTab::RequestHeaders => {
                    if let RequestHeaderFocus::Header(header_index) =
                        self.main_state.current_header
                        && let Some((header_name, header_value)) =
                            self.main_state.headers.get_mut(header_index)
                        && self.main_state.current_header_cursor > 0
                    {
                        match self.main_state.current_header_section {
                            HeaderSection::Name => {
                                header_name.remove(
                                    self.main_state.current_header_cursor.saturating_sub(1),
                                );
                            }
                            HeaderSection::Value => {
                                header_value.remove(
                                    self.main_state.current_header_cursor.saturating_sub(1),
                                );
                            }
                            _ => {}
                        }

                        self.main_state.current_header_cursor =
                            self.main_state.current_header_cursor.saturating_sub(1);
                    }
                }
                MainTab::RequestBody => {
                    let body_cursor: &mut BodyCursor = &mut self.main_state.request_body_cursor;
                    let body = &mut self.main_state.request_body;
                    body_cursor.column = body_cursor
                        .column
                        .min(body.get(body_cursor.line).unwrap_or(&String::new()).len());

                    let body_len = body.len();
                    if let Some(line) = body.get_mut(body_cursor.line) {
                        if body_len > 1 && body_cursor.column == 0 && body_cursor.line > 0 {
                            let carryover = body.remove(body_cursor.line);
                            body_cursor.line = body_cursor.line.saturating_sub(1);
                            body_cursor.column =
                                body.get(body_cursor.line).unwrap_or(&String::new()).len();
                            if let Some(current_line) = body.get_mut(body_cursor.line) {
                                current_line.push_str(&carryover);
                            }
                        } else {
                            *line = [
                                line.get(0..body_cursor.column.saturating_sub(1))
                                    .unwrap_or_default(),
                                line.get((body_cursor.column)..).unwrap_or_default(),
                            ]
                            .join("");
                            body_cursor.column = body_cursor.column.saturating_sub(1);
                        }
                    } else {
                        body_cursor.line = body.len().saturating_sub(1);
                    }
                }
                MainTab::ResponseData => {}
                MainTab::ResponseBody => {}
            },
            KeyCode::Enter => match main_tab {
                MainTab::RequestHeaders => match self.main_state.current_header {
                    RequestHeaderFocus::Header(index)
                        if self.main_state.current_header_section == HeaderSection::Delete =>
                    {
                        self.main_state.headers.remove(index);
                        if index >= self.main_state.headers.len() {
                            self.main_state.current_header =
                                if !self.main_state.headers.is_empty() {
                                    RequestHeaderFocus::Header(self.main_state.headers.len() - 1)
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
                    RequestHeaderFocus::Add => self
                        .main_state
                        .headers
                        .push((String::new(), String::new())),
                },
                MainTab::RequestBody if !self.editing => {
                    self.editing = true;
                }
                MainTab::RequestBody => {
                    let body_cursor = &mut self.main_state.request_body_cursor;
                    let prev_line = self
                        .main_state
                        .request_body
                        .get(body_cursor.line)
                        .cloned()
                        .unwrap_or(String::new());

                    let before_string = prev_line
                        .get(0..body_cursor.column.min(prev_line.len()))
                        .unwrap_or_default()
                        .to_string();

                    let mut after_string_indent = String::new();
                    for i in 0..before_string.len() {
                        if before_string.get(i..=i) == Some(" ") {
                            after_string_indent.push(' ');
                        } else {
                            break;
                        }
                    }

                    let body = &mut self.main_state.request_body;

                    if let Some(line) = body.get_mut(body_cursor.line) {
                        *line = before_string;
                    };
                    let after_string = prev_line
                        .get(body_cursor.column..)
                        .unwrap_or_default()
                        .to_string();

                    if after_string.starts_with("]") || after_string.starts_with("}") {
                        let middle_string_indent = format!("{}    ", after_string_indent);
                        let middle_string_indent_len = middle_string_indent.len();
                        body.insert(body_cursor.line + 1, middle_string_indent);
                        let after_string = format!("{}{}", after_string_indent, after_string);

                        body.insert(body_cursor.line + 2, after_string);

                        body_cursor.line += 1;
                        body_cursor.column = middle_string_indent_len;
                    } else {
                        let after_string = format!("{}{}", after_string_indent, after_string);
                        body.insert(body_cursor.line + 1, after_string);

                        body_cursor.line += 1;
                        body_cursor.column = after_string_indent.len();
                    }
                }
                MainTab::ResponseData => {},
                MainTab::ResponseBody => {}
            },
            KeyCode::Up => match main_tab {
                MainTab::RequestHeaders => {
                    if let RequestHeaderFocus::Header(header_num) =
                        self.main_state.current_header
                        && !self.editing
                    {
                        self.main_state.current_header =
                            RequestHeaderFocus::Header(header_num.saturating_sub(1))
                    } else if self.main_state.current_header == RequestHeaderFocus::Add
                        && !self.main_state.headers.is_empty()
                    {
                        self.main_state.current_header = RequestHeaderFocus::Header(
                            self.main_state.headers.len().saturating_sub(1),
                        )
                    }
                }
                MainTab::RequestBody if self.editing => {
                    let body_cursor = &mut self.main_state.request_body_cursor;

                    body_cursor.line = body_cursor.line.saturating_sub(1);
                }
                _ => {}
            },
            KeyCode::Down => match main_tab {
                MainTab::RequestHeaders => {
                    if let RequestHeaderFocus::Header(header_num) =
                        self.main_state.current_header
                        && !self.editing
                    {
                        self.main_state.current_header =
                            if header_num >= self.main_state.headers.len() - 1 {
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
                MainTab::RequestBody if self.editing => {
                    let body_cursor = &mut self.main_state.request_body_cursor;

                    body_cursor.line =
                        (body_cursor.line + 1).min(self.main_state.request_body.len() - 1);
                }
                _ => {}
            },
            KeyCode::Left => match main_tab {
                MainTab::RequestHeaders if self.editing => {
                    if let RequestHeaderFocus::Header(_) = self.main_state.current_header
                        && self.main_state.current_header_section != HeaderSection::Delete
                    {
                        self.main_state.current_header_cursor =
                            self.main_state.current_header_cursor.saturating_sub(1);
                    }
                }
                MainTab::RequestHeaders => {
                    self.main_state.current_header_section.decrement();
                }
                MainTab::RequestBody if self.editing => {
                    let body_cursor = &mut self.main_state.request_body_cursor;

                    body_cursor.column = body_cursor.column.min(
                        self.main_state
                            .request_body
                            .get(body_cursor.line)
                            .unwrap_or(&String::new())
                            .len(),
                    );

                    if body_cursor.column < 1 && body_cursor.line > 0 {
                        body_cursor.line = body_cursor.line.saturating_sub(1);

                        let body_line = self
                            .main_state
                            .request_body
                            .get(body_cursor.line)
                            .unwrap_or(&String::new())
                            .len();

                        body_cursor.column = body_line;
                    } else {
                        body_cursor.column = body_cursor.column.saturating_sub(1);
                    }
                }
                _ => {}
            },
            KeyCode::Right => match main_tab {
                MainTab::RequestHeaders if self.editing => match self.main_state.current_header {
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
                MainTab::RequestHeaders => {
                    self.main_state.current_header_section.increment();
                }
                MainTab::RequestBody => {
                    if self.editing {
                        let body_cursor = &mut self.main_state.request_body_cursor;

                        body_cursor.column = body_cursor.column.min(
                            self.main_state
                                .request_body
                                .get(body_cursor.line)
                                .unwrap_or(&String::new())
                                .len(),
                        );

                        let body_line = self
                            .main_state
                            .request_body
                            .get(body_cursor.line)
                            .cloned()
                            .unwrap_or(String::new());

                        if body_cursor.line < self.main_state.request_body.len().saturating_sub(1)
                            && body_cursor.column >= body_line.len()
                        {
                            body_cursor.line = (body_cursor.line + 1)
                                .min(self.main_state.request_body.len().saturating_sub(1));
                            body_cursor.column = 0;
                        } else {
                            body_cursor.column = (body_cursor.column + 1).min(body_line.len());
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Tab if main_tab == MainTab::RequestBody && self.editing => {
                let body_cursor: &mut BodyCursor = &mut self.main_state.request_body_cursor;
                if let Some(line) = self.main_state.request_body.get_mut(body_cursor.line) {
                    line.insert_str(0, "    ");
                    body_cursor.column = (body_cursor.column + 4).min(line.len());
                }
            }
            KeyCode::BackTab if main_tab == MainTab::RequestBody && self.editing => {
                let body_cursor: &mut BodyCursor = &mut self.main_state.request_body_cursor;
                if let Some(line) = self.main_state.request_body.get_mut(body_cursor.line) {
                    for _ in 0..4 {
                        if let Some(" ") = line.get(0..=0) {
                            line.remove(0);
                            body_cursor.column = body_cursor.column.saturating_sub(1);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
