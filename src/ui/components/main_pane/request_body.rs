use serde_json::Value;

use ratatui::{
    layout::{Constraint, Layout, Position, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::Application;

impl Application {
    pub fn render_request_body_pane(&self, frame: &mut Frame, area: Rect) {
        let ref_request_body = &self.main_state.request_body;

        let request_body_layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(area);
        let (body_panel_pre, status_panel) = (request_body_layout[0], request_body_layout[1]);
        let body_panel_pre =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).split(body_panel_pre);
        let (body_panel, scroll_panel) = (body_panel_pre[0], body_panel_pre[1]);
        let (status_text, error_position) = if ref_request_body.len_chars() > 0 {
            match serde_json::from_str::<Value>(&ref_request_body.to_string()) {
                Ok(_) => (
                    Span::styled(
                        "Valid".to_string(),
                        Style::new().fg(self.settings.theme.green),
                    ),
                    None,
                ),
                Err(e) => (
                    Span::styled(
                        format!("Invalid - Line {} Column {}", e.line(), e.column()),
                        Style::new().fg(self.settings.theme.red),
                    ),
                    Some((e.line(), e.column())),
                ),
            }
        } else {
            (Span::raw(""), None)
        };

        let viewable_line_count = body_panel.height as usize - 2; // body panel has padding

        let mut offset = self
            .main_state
            .request_body_cursor
            .as_line(ref_request_body)
            .saturating_sub(viewable_line_count / 2);

        if offset + viewable_line_count > ref_request_body.len_lines() {
            offset = ref_request_body
                .len_lines()
                .saturating_sub(viewable_line_count);
        }

        let start = ref_request_body
            .line_to_char(offset)
            .min(ref_request_body.len_chars());

        let end_line_index =
            (offset + viewable_line_count).min(ref_request_body.len_lines().saturating_sub(1));
        let end_line_len = ref_request_body.line(end_line_index).len_chars();

        let end = ref_request_body.line_to_char(end_line_index) + end_line_len;

        let trimmed = ref_request_body.slice(start..end).lines();

        let mut in_quote_scope = false;

        frame.render_widget(
            Paragraph::new(Text::from_iter(trimmed.enumerate().map(
                |(line_idx, line)| {
                    let spans = line.chars().enumerate().map(|(char_idx, c)| {
                        let style = if let Some((error_line, error_column)) = error_position
                            && line_idx + start == error_line.saturating_sub(1)
                            && char_idx == error_column.saturating_sub(1)
                        {
                            Style::new()
                                .fg(self.settings.theme.text_inverse)
                                .bg(self.settings.theme.red)
                        } else if c == '{' || c == '}' {
                            Style::new().fg(self.settings.theme.accent)
                        } else if c == '[' || c == ']' {
                            Style::new().fg(self.settings.theme.active_element)
                        } else if c == '"' {
                            in_quote_scope = !in_quote_scope;
                            Style::new().fg(self.settings.theme.green)
                        } else if in_quote_scope {
                            Style::new().fg(self.settings.theme.green)
                        } else {
                            Style::new()
                        };

                        Span::styled(c.to_string(), style)
                    });

                    Line::from_iter(spans)
                },
            )))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1))),
            body_panel,
        );
        frame.render_widget(
            Paragraph::new(status_text).block(Block::new().padding(Padding::new(1, 1, 0, 0))),
            status_panel,
        );

        let scrollbar_position = self
            .main_state
            .request_body_cursor
            .as_line(ref_request_body) as f64
            / ref_request_body.len_lines().saturating_sub(1) as f64
            * viewable_line_count as f64;

        let scrollbar_position = (scrollbar_position as u16).min(viewable_line_count as u16);

        let scrollbar =
            Paragraph::new(Line::styled("█", Style::new().fg(self.settings.theme.text)));

        if ref_request_body.len_lines() > viewable_line_count {
            frame.render_widget(
                scrollbar,
                Rect {
                    y: scroll_panel.y + scrollbar_position,
                    ..scroll_panel
                },
            );
        }

        if self.editing {
            let cursor_position = Position {
                x: self
                    .main_state
                    .request_body_cursor
                    .as_char_in_line(ref_request_body) as u16
                    + body_panel.x
                    + 1,
                y: self
                    .main_state
                    .request_body_cursor
                    .as_line(ref_request_body) as u16
                    + body_panel.y
                    + 1
                    - offset as u16,
            };

            frame.set_cursor_position(cursor_position);
        }
    }
}
