use std::iter;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{
    Application,
    state::{Panel, RequestTab},
    ui::{components::badge::badge, palette},
};

impl Application {
    pub fn render_request_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = if self.state.focused_panel == Panel::Request {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

        let (headers_fg, headers_bg) = if self.state.current_request_tab == RequestTab::Headers {
            (palette::SAPPHIRE, palette::SURFACE2)
        } else {
            (palette::SUBTEXT0, palette::SURFACE0)
        };

        let (body_fg, body_bg) = if self.state.current_request_tab == RequestTab::Body {
            (palette::SAPPHIRE, palette::SURFACE2)
        } else {
            (palette::SUBTEXT0, palette::SURFACE0)
        };

        let (settings_fg, settings_bg) = if self.state.current_request_tab == RequestTab::Settings {
            (palette::SAPPHIRE, palette::SURFACE2)
        } else {
            (palette::SUBTEXT0, palette::SURFACE0)
        };

        let tabs = [
            badge("Headers", Some(headers_fg), headers_bg),
            badge("Body", Some(body_fg), body_bg),
            badge("Settings", Some(settings_fg), settings_bg),
        ]
        .concat();

        match self.state.current_request_tab {
            RequestTab::Headers => self.render_request_headers_pane(frame, content_area),
            RequestTab::Body => self.render_request_body_pane(frame, content_area),
            RequestTab::Settings => self.render_request_settings_pane(frame, content_area),
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
        let header_elements = self
            .state
            .request_headers
            .iter()
            .cloned()
            .flat_map(|(name, value)| {
                let padding = 1;
                let name_field_width = 16;

                // magic number guide: 2 is name field badge side characters, 1 is colon separator
                let value_field_width = area.width - (padding * 2) - name_field_width - 2 - 1;

                let name_padding_len = name_field_width as usize - name.len();
                let name_padding = iter::repeat_n(' ', name_padding_len).collect::<String>();
                let value_padding_len = value_field_width as usize - value.len();
                let value_padding = iter::repeat_n(' ', value_padding_len).collect::<String>();

                [
                    Line::from_iter(
                        [
                            badge(format!("{}{}", name, name_padding), None, palette::SURFACE0),
                            vec![Span::styled(":", Style::new().fg(palette::TEXT))],
                            badge(
                                format!("{}{}", value, value_padding),
                                None,
                                palette::SURFACE0,
                            ),
                        ]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>(),
                    ),
                    Line::from(""),
                ]
            })
            .collect::<Vec<_>>();

        let add_header_button = Line::from_iter(badge(
            "Add Header",
            Some(palette::SUBTEXT0),
            palette::SURFACE0,
        ));

        let list = Paragraph::new(Text::from_iter(
            [header_elements, vec![add_header_button, Line::from("")]]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        ))
        .block(Block::new().padding(Padding::new(1, 1, 1, 1)));

        frame.render_widget(list, area);
    }

    pub fn render_request_body_pane(&self, frame: &mut Frame, area: Rect) {}

    pub fn render_request_settings_pane(&self, frame: &mut Frame, area: Rect) {}
}
