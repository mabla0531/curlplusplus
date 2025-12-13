use crate::Application;
use crate::{BodyCursor, HeaderSection, RequestHeaderFocus, RequestTab};

use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_request_pane_input(&mut self, event: KeyEvent, request_tab: RequestTab) {
        match event.code {
            KeyCode::Char(character) => match request_tab {
                RequestTab::Headers if self.editing => {
                    if let RequestHeaderFocus::Header(header) = self.request_state.current_header
                        && let Some(name) = self.request_state.headers.get_mut(header)
                    {
                        match self.request_state.current_header_section {
                            HeaderSection::Name => name.0.push(character),
                            HeaderSection::Value => name.1.push(character),
                            _ => {}
                        }
                    }
                }
                RequestTab::Body if self.editing => {
                    let body_cursor: &mut BodyCursor = &mut self.request_state.body_cursor;
                    if let Some(line) = self.request_state.body.get_mut(body_cursor.line) {
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
            KeyCode::Backspace if self.editing => match request_tab {
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
            },
            KeyCode::Enter => match request_tab {
                RequestTab::Headers => match self.request_state.current_header {
                    RequestHeaderFocus::Header(index)
                        if self.request_state.current_header_section == HeaderSection::Delete =>
                    {
                        self.request_state.headers.remove(index);
                        if index >= self.request_state.headers.len() {
                            self.request_state.current_header =
                                if !self.request_state.headers.is_empty() {
                                    RequestHeaderFocus::Header(self.request_state.headers.len() - 1)
                                } else {
                                    RequestHeaderFocus::Add
                                }
                        }
                    }
                    RequestHeaderFocus::Header(_) => {
                        self.editing = !self.editing;
                    }
                    RequestHeaderFocus::Add => self
                        .request_state
                        .headers
                        .push((String::new(), String::new())),
                },
                RequestTab::Body if !self.editing => {
                    self.editing = true;
                }
                RequestTab::Body => {
                    let body_cursor = &mut self.request_state.body_cursor;
                    let prev_line = self
                        .request_state
                        .body
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

                    let body = &mut self.request_state.body;

                    if let Some(line) = body.get_mut(body_cursor.line) {
                        *line = before_string;
                    };
                    let after_string = prev_line
                        .get(body_cursor.column..)
                        .unwrap_or_default()
                        .to_string();

                    if after_string.starts_with("]")
                        || after_string.starts_with("}")
                        || after_string.starts_with("\"")
                    {
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
            },
            KeyCode::Up => match request_tab {
                RequestTab::Headers => {
                    self.request_state.current_header = match self.request_state.current_header {
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
            KeyCode::Down => match request_tab {
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
            KeyCode::Left => match request_tab {
                RequestTab::Headers if self.editing => {
                    // TODO Move header cursor
                }
                RequestTab::Headers => {
                    self.request_state.current_header_section.decrement();
                }
                RequestTab::Body => {
                    let body_cursor = &mut self.request_state.body_cursor;

                    body_cursor.column = body_cursor.column.min(
                        self.request_state
                            .body
                            .get(body_cursor.line)
                            .unwrap_or(&String::new())
                            .len(),
                    );

                    if body_cursor.column < 1 && body_cursor.line > 0 {
                        body_cursor.line = body_cursor.line.saturating_sub(1);

                        let body_line = self
                            .request_state
                            .body
                            .get(body_cursor.line)
                            .unwrap_or(&String::new())
                            .len();

                        body_cursor.column = body_line;
                    } else {
                        body_cursor.column = body_cursor.column.saturating_sub(1);
                    }
                }
            },
            KeyCode::Right => match request_tab {
                RequestTab::Headers if self.editing => {
                    // TODO Move header cursor
                }
                RequestTab::Headers => {
                    self.request_state.current_header_section.increment();
                }
                RequestTab::Body => {
                    let body_cursor = &mut self.request_state.body_cursor;

                    body_cursor.column = body_cursor.column.min(
                        self.request_state
                            .body
                            .get(body_cursor.line)
                            .unwrap_or(&String::new())
                            .len(),
                    );

                    let body_line = self
                        .request_state
                        .body
                        .get(body_cursor.line)
                        .cloned()
                        .unwrap_or(String::new());

                    if body_cursor.line < self.request_state.body.len().saturating_sub(1)
                        && body_cursor.column >= body_line.len()
                    {
                        body_cursor.line = (body_cursor.line + 1)
                            .min(self.request_state.body.len().saturating_sub(1));
                        body_cursor.column = 0;
                    } else {
                        body_cursor.column = (body_cursor.column + 1).min(body_line.len());
                    }
                }
            },
            KeyCode::Tab if request_tab == RequestTab::Body && self.editing => {
                let body_cursor: &mut BodyCursor = &mut self.request_state.body_cursor;
                if let Some(line) = self.request_state.body.get_mut(body_cursor.line) {
                    line.insert_str(0, "    ");
                    body_cursor.column = (body_cursor.column + 4).min(line.len());
                }
            }
            KeyCode::BackTab if request_tab == RequestTab::Body && self.editing => {
                let body_cursor: &mut BodyCursor = &mut self.request_state.body_cursor;
                if let Some(line) = self.request_state.body.get_mut(body_cursor.line) {
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
