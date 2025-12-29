use crate::Application;

use crossterm::event::{KeyCode, KeyEvent};

impl Application {
    pub fn handle_request_body_input(&mut self, event: KeyEvent) {
        let mut_request_body = &mut self.main_state.request_body;
        let mut_request_body_cursor = &mut self.main_state.request_body_cursor;

        match event.code {
            KeyCode::Char(character) if self.editing => {
                mut_request_body.insert_char(mut_request_body_cursor.position, character);
                mut_request_body_cursor.position += 1;
                mut_request_body_cursor.target_character =
                    mut_request_body_cursor.as_char_in_line(mut_request_body);

                match character {
                    '{' => mut_request_body.insert_char(mut_request_body_cursor.position, '}'),
                    '[' => mut_request_body.insert_char(mut_request_body_cursor.position, ']'),
                    '\"' => mut_request_body.insert_char(mut_request_body_cursor.position, '\"'),
                    _ => {}
                }
            }

            KeyCode::Backspace if self.editing => {
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

            KeyCode::Enter if !self.editing => {
                self.editing = true;
            }
            KeyCode::Enter => {
                let mut indentation = mut_request_body
                    .line(mut_request_body_cursor.as_line(mut_request_body))
                    .chars()
                    .take_while(|c| *c == ' ')
                    .collect::<String>();

                if ['{', '['].contains(
                    &mut_request_body.char(mut_request_body_cursor.position.saturating_sub(1)),
                ) && ['}', ']']
                    .contains(&mut_request_body.char(mut_request_body_cursor.position))
                {
                    mut_request_body.insert(
                        mut_request_body_cursor.position,
                        format!("\n{}", indentation).as_str(),
                    );
                    indentation.push_str("    ");
                }

                mut_request_body.insert_char(mut_request_body_cursor.position, '\n');
                mut_request_body_cursor.position += 1;

                mut_request_body.insert(mut_request_body_cursor.position, indentation.as_str());
                mut_request_body_cursor.position += indentation.len();

                mut_request_body_cursor.target_character =
                    mut_request_body_cursor.as_char_in_line(mut_request_body);
            }

            KeyCode::Up if self.editing => {
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

            KeyCode::Down if self.editing => {
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

            KeyCode::Left if self.editing => {
                mut_request_body_cursor.position =
                    mut_request_body_cursor.position.saturating_sub(1);
                mut_request_body_cursor.target_character =
                    mut_request_body_cursor.as_char_in_line(mut_request_body);
            }

            KeyCode::Right => {
                if self.editing {
                    mut_request_body_cursor.position =
                        (mut_request_body_cursor.position + 1).min(mut_request_body.len_chars());
                    mut_request_body_cursor.target_character =
                        mut_request_body_cursor.as_char_in_line(mut_request_body);
                }
            }

            KeyCode::Tab if self.editing => {
                mut_request_body.insert(
                    mut_request_body_cursor.as_line_start(mut_request_body),
                    "    ",
                );
                mut_request_body_cursor.position += 4;
            }
            _ => {}
        }
    }

    pub fn handle_request_body_paste(&mut self, text: String) {
        let mut_request_body = &mut self.main_state.request_body;
        let mut_request_body_cursor = &mut self.main_state.request_body_cursor;

        mut_request_body.insert(mut_request_body_cursor.position, text.as_str());
        //mut_request_body_cursor.position += text.len();
        //mut_request_body_cursor.target_character =
        //    mut_request_body_cursor.as_char_in_line(mut_request_body);
    }
}
