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
    state::{HeaderSection, Panel, RequestHeaderFocus, MainTab},
    ui::{components::badge::badge, palette},
};

const HEADER_NAME_FIELD_WIDTH: usize = 28;

impl Application {
    pub fn render_main_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = match self.focused_panel {
            Panel::Main(_) => Style::new().fg(palette::SKY),
            _ => Style::new().fg(palette::TEXT),
        };

        let (request_headers_tab_fg, request_headers_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::RequestHeaders) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let (request_body_tab_fg, request_body_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::RequestBody) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let (response_data_tab_fg, response_data_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::ResponseData) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let (response_body_tab_fg, response_body_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::ResponseBody) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let tabs = [
            badge("Request Headers", Some(request_headers_tab_fg), request_headers_tab_bg),
            badge("Request Body", Some(request_body_tab_fg), request_body_tab_bg),
            badge("Response Data", Some(response_data_tab_fg), response_data_tab_bg),
            badge("Response Body", Some(response_body_tab_fg), response_body_tab_bg),
        ]
        .concat();

        match self.focused_panel {
            Panel::Main(MainTab::RequestHeaders) => {
                self.render_request_headers_pane(frame, content_area)
            }
            Panel::Main(MainTab::RequestBody) => self.render_request_body_pane(frame, content_area),
            Panel::Main(MainTab::ResponseData) => {},
            Panel::Main(MainTab::ResponseBody) => {},
            _ => {}
        }

        let tabs_paragraph = Paragraph::new(Line::from_iter(tabs));

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
            area,
        );
        frame.render_widget(tabs_paragraph, tabs_area);
    }

    pub fn render_request_headers_pane(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(area);
        let (untrimmed_headers_panel, add_button_panel) = (layout[0], layout[1]);
        let header_layout = Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)])
            .split(untrimmed_headers_panel);
        let (headers_panel, scroll_panel) = (header_layout[0], header_layout[1]);
        let viewable_header_count = (headers_panel.height as usize / 2).saturating_sub(1);

        let index = match self.main_state.current_header {
            RequestHeaderFocus::Header(index) => index,
            RequestHeaderFocus::Add => self.main_state.headers.len().saturating_sub(1),
        };

        let mut offset = index.saturating_sub(viewable_header_count / 2);

        if offset + viewable_header_count > self.main_state.headers.len() {
            offset = self
                .main_state
                .headers
                .len()
                .saturating_sub(viewable_header_count);
        }

        let begin = offset;
        let end = (offset + viewable_header_count).min(self.main_state.headers.len());
        let trimmed = &self.main_state.headers[begin..end];

        let header_elements = trimmed
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(index, (name, value))| {
                header_line(
                    name,
                    value,
                    self.main_state.current_header == index + offset,
                    self.main_state.current_header_section.clone(),
                    area,
                )
            })
            .collect::<Vec<_>>();

        let header_paragraph = Paragraph::new(Text::from_iter(header_elements))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1)));

        let (add_button_fg, add_button_bg) =
            match (&self.focused_panel, &self.main_state.current_header) {
                (Panel::Main(MainTab::RequestHeaders), RequestHeaderFocus::Add) => {
                    (palette::SUBTEXT1, palette::SURFACE1)
                }
                _ => (palette::SUBTEXT0, palette::BASE),
            };

        let add_header_button =
            Line::from_iter(badge("Add Header", Some(add_button_fg), add_button_bg));

        let scrollbar_position = index as f64
            / self.main_state.headers.len().saturating_sub(1) as f64
            * headers_panel.height as f64;

        let scrollbar_position = (scrollbar_position as u32).min(headers_panel.height as u32);

        let scrollbar = Paragraph::new(Line::styled("█", Style::new().fg(palette::TEXT)));

        frame.render_widget(header_paragraph, headers_panel);
        frame.render_widget(add_header_button, add_button_panel);
        if self.main_state.headers.len() > viewable_header_count {
            frame.render_widget(
                scrollbar,
                Rect {
                    y: scroll_panel.y + scrollbar_position as u16,
                    ..scroll_panel
                },
            );
        }

        let cursor_horizontal_offset =
            if self.main_state.current_header_section == HeaderSection::Value {
                HEADER_NAME_FIELD_WIDTH + 3 // I think this is caused by gap and padding
            } else {
                0
            };

        if self.editing
            && let RequestHeaderFocus::Header(current_header) = self.main_state.current_header
        {
            frame.set_cursor_position(Position::from((
                // 2 for padding
                headers_panel.x
                    + 2
                    + (self.main_state.current_header_cursor + cursor_horizontal_offset) as u16,
                // 1 for padding and *2 for spacing between each header
                headers_panel.y + ((current_header - offset) * 2) as u16 + 1,
            )));
        }
    }

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
                y: self.main_state.request_body_cursor.line as u16 + body_panel.y + 1 - offset as u16,
            };

            frame.set_cursor_position(cursor_position);
        }
    }
}

fn header_line<'a>(
    name: String,
    value: String,
    focused: bool,
    section: HeaderSection,
    area: Rect,
) -> [Line<'a>; 2] {
    let padding = 1_usize;

    // magic number guide: 2 is name field badge side characters, 1 is colon separator
    let value_field_width =
        (area.width as usize).saturating_sub(padding * 2 + HEADER_NAME_FIELD_WIDTH + 2 + 1);

    let name_padding_len = HEADER_NAME_FIELD_WIDTH.saturating_sub(name.len());
    let name_padding = iter::repeat_n(' ', name_padding_len).collect::<String>();
    let value_padding_len = value_field_width
        .saturating_sub(value.len() + 6) // 6 == trashcan badge (when this is 5 (the theoretical width of a padded UTF-16 character) it doesn't render the delete badge (no idea why!))
        .min(value_field_width);

    let value_padding = iter::repeat_n(' ', value_padding_len).collect::<String>();

    let name_badge = badge(
        format!("{}{}", name, name_padding),
        Some(palette::TEXT),
        if focused && section == HeaderSection::Name {
            palette::SURFACE1
        } else if focused {
            palette::SURFACE0
        } else {
            palette::BASE
        },
    );

    let separator = vec![Span::styled(":", Style::new().fg(palette::TEXT))];

    let value_badge = badge(
        format!("{}{}", value, value_padding),
        Some(palette::TEXT),
        if focused && section == HeaderSection::Value {
            palette::SURFACE1
        } else if focused {
            palette::SURFACE0
        } else {
            palette::BASE
        },
    );

    let delete_badge = badge(
        "",
        if focused && section == HeaderSection::Delete {
            Some(palette::RED)
        } else {
            Some(palette::MAROON)
        },
        if focused && section == HeaderSection::Delete {
            palette::SURFACE1
        } else if focused {
            palette::SURFACE0
        } else {
            palette::BASE
        },
    );

    [
        Line::from_iter(
            [name_badge, separator, value_badge, delete_badge]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        ),
        Line::from(""),
    ]
}
