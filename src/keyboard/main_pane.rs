use crate::Application;
use crate::{HeaderSection, MainTab, RequestHeaderFocus};

use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_main_pane_input(&mut self, event: KeyEvent, main_tab: MainTab) {
        let mut_request_body = &mut self.main_state.request_body;
        let mut_request_body_cursor = &mut self.main_state.request_body_cursor;

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
                    mut_request_body.insert_char(mut_request_body_cursor.position, character);
                    mut_request_body_cursor.position += 1;
                    mut_request_body_cursor.target_character =
                        mut_request_body_cursor.as_char_in_line(mut_request_body);

                    match character {
                        '{' => mut_request_body.insert_char(mut_request_body_cursor.position, '}'),
                        '[' => mut_request_body.insert_char(mut_request_body_cursor.position, ']'),
                        '\"' => {
                            mut_request_body.insert_char(mut_request_body_cursor.position, '\"')
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            KeyCode::Backspace if self.editing => match main_tab {
                MainTab::RequestHeaders => {
                    if let RequestHeaderFocus::Header(header_index) = self.main_state.current_header
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
                    if mut_request_body.len_chars() > 0 && mut_request_body_cursor.position > 0 {
                        mut_request_body.remove(
                            mut_request_body_cursor.position.saturating_sub(1)
                                ..mut_request_body_cursor.position,
                        );
                        mut_request_body_cursor.position =
                            mut_request_body_cursor.position.saturating_sub(1);
                        mut_request_body_cursor.target_character =
                            mut_request_body_cursor.as_char_in_line(mut_request_body);
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
                            self.main_state.current_header = if !self.main_state.headers.is_empty()
                            {
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
                MainTab::RequestBody if !self.editing => {
                    self.editing = true;
                }
                MainTab::RequestBody => {
                    mut_request_body.insert_char(mut_request_body_cursor.position, '\n');
                    mut_request_body_cursor.position += 1;
                    mut_request_body_cursor.target_character =
                        mut_request_body_cursor.as_char_in_line(mut_request_body);
                }
                MainTab::ResponseData => {}
                MainTab::ResponseBody => {}
            },
            KeyCode::Up => match main_tab {
                MainTab::RequestHeaders => {
                    if let RequestHeaderFocus::Header(header_num) = self.main_state.current_header
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
                    let new_line = mut_request_body_cursor
                        .as_line(mut_request_body)
                        .saturating_sub(1);
                    let new_line_start = mut_request_body.line_to_char(new_line);

                    let new_line_len = mut_request_body.line(new_line).len_chars();
                    let new_line_len = if let Some('\n') = mut_request_body.line(new_line).get_char(
                        mut_request_body
                            .line(new_line)
                            .len_chars()
                            .saturating_sub(1),
                    ) {
                        new_line_len.saturating_sub(1)
                    } else {
                        new_line_len
                    };

                    mut_request_body_cursor.position =
                        new_line_start + mut_request_body_cursor.target_character.min(new_line_len);
                }
                _ => {}
            },
            KeyCode::Down => match main_tab {
                MainTab::RequestHeaders => {
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
                MainTab::RequestBody if self.editing => {
                    let new_line = (mut_request_body_cursor.as_line(mut_request_body) + 1)
                        .min(mut_request_body.len_lines().saturating_sub(1));
                    let new_line_start = mut_request_body.line_to_char(new_line);

                    let new_line_len = mut_request_body.line(new_line).len_chars();
                    let new_line_len = if let Some('\n') = mut_request_body.line(new_line).get_char(
                        mut_request_body
                            .line(new_line)
                            .len_chars()
                            .saturating_sub(1),
                    ) {
                        new_line_len.saturating_sub(1)
                    } else {
                        new_line_len
                    };

                    mut_request_body_cursor.position =
                        new_line_start + mut_request_body_cursor.target_character.min(new_line_len);
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
                    mut_request_body_cursor.position =
                        mut_request_body_cursor.position.saturating_sub(1);
                    mut_request_body_cursor.target_character =
                        mut_request_body_cursor.as_char_in_line(mut_request_body);
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
                        mut_request_body_cursor.position = (mut_request_body_cursor.position + 1)
                            .min(mut_request_body.len_chars());
                        mut_request_body_cursor.target_character =
                            mut_request_body_cursor.as_char_in_line(mut_request_body);
                    }
                }
                _ => {}
            },
            KeyCode::Tab if main_tab == MainTab::RequestBody && self.editing => {
                mut_request_body.insert(
                    mut_request_body_cursor.as_line_start(mut_request_body),
                    "    ",
                );
                mut_request_body_cursor.position += 4;
            }
            _ => {}
        }
    }
}
