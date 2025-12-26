use serde_json::Value;
use std::iter;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{
    Application,
    state::{HeaderSection, MainTab, Panel, RequestHeaderFocus},
    ui::{components::badge::badge, palette},
};

impl Application {
    pub fn render_request_body_pane(&self, frame: &mut Frame, area: Rect) {
        let request_body_layout =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(area);
        let (body_panel_pre, status_panel) = (request_body_layout[0], request_body_layout[1]);
        let body_panel_pre =
            Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)]).split(body_panel_pre);
        let (body_panel, scroll_panel) = (body_panel_pre[0], body_panel_pre[1]);
        let (status_text, error_position) = if !self.main_state.request_body.is_empty() {
            match serde_json::from_str::<Value>(&self.main_state.request_body.join("\n")) {
                Ok(_) => (
                    Span::styled("Valid".to_string(), Style::new().fg(palette::GREEN)),
                    None,
                ),
                Err(e) => (
                    Span::styled(
                        format!("Invalid - Line {} Column {}", e.line(), e.column()),
                        Style::new().fg(palette::RED),
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
            .line
            .saturating_sub(viewable_line_count / 2);

        if offset + viewable_line_count > self.main_state.request_body.len() {
            offset = self
                .main_state
                .request_body
                .len()
                .saturating_sub(viewable_line_count);
        }

        let begin = offset;
        let end = (offset + viewable_line_count).min(self.main_state.request_body.len());
        let trimmed = &self.main_state.request_body[begin..end];

        let mut in_quote_scope = false;

        frame.render_widget(
            Paragraph::new(Text::from_iter(trimmed.iter().enumerate().map(
                |(line_idx, line)| {
                    let spans = line.chars().enumerate().map(|(char_idx, c)| {
                        let style = if let Some((error_line, error_column)) = error_position
                            && line_idx + offset == error_line.saturating_sub(1)
                            && char_idx == error_column.saturating_sub(1)
                        {
                            Style::new().bg(palette::RED)
                        } else if c == '{' || c == '}' {
                            Style::new().fg(palette::PEACH)
                        } else if c == '[' || c == ']' {
                            Style::new().fg(palette::TEAL)
                        } else if c == '"' {
                            in_quote_scope = !in_quote_scope;
                            Style::new().fg(palette::GREEN)
                        } else if in_quote_scope {
                            Style::new().fg(palette::GREEN)
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

        let scrollbar_position = self.main_state.request_body_cursor.line as f64
            / self.main_state.request_body.len().saturating_sub(1) as f64
            * viewable_line_count as f64;

        let scrollbar_position = (scrollbar_position as u16).min(viewable_line_count as u16);

        let scrollbar = Paragraph::new(Line::styled("█", Style::new().fg(palette::TEXT)));

        if self.main_state.request_body.len() > viewable_line_count as usize {
            frame.render_widget(
                scrollbar,
                Rect {
                    y: scroll_panel.y + scrollbar_position as u16,
                    ..scroll_panel
                },
            );
        }

        if self.editing {
            let cursor_position = Position {
                x: (self.main_state.request_body_cursor.column).min(
                    self.main_state
                        .request_body
                        .get(self.main_state.request_body_cursor.line)
                        .unwrap_or(&String::new())
                        .len(),
                ) as u16
                    + body_panel.x
                    + 1,
                y: self.main_state.request_body_cursor.line as u16 + body_panel.y + 1
                    - offset as u16,
            };

            frame.set_cursor_position(cursor_position);
        }
    }
}
