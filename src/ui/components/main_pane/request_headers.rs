use std::iter;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph},
};

use crate::{
    Application,
    state::{HeaderSection, MainTab, Panel, RequestHeaderFocus},
    ui::{components::badge::badge, palette},
};

const HEADER_NAME_FIELD_WIDTH: usize = 28;

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

impl Application {
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
}
