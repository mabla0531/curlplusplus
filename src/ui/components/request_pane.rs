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
    state::{Panel, RequestHeaderFocus, RequestTab},
    ui::{components::badge::badge, palette},
};

impl Application {
    pub fn render_request_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = match self.focused_panel {
            Panel::Request(_) => Style::new().fg(palette::SKY),
            _ => Default::default(),
        };

        let (headers_fg, headers_bg) = match self.focused_panel {
            Panel::Request(RequestTab::Headers) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let (body_fg, body_bg) = match self.focused_panel {
            Panel::Request(RequestTab::Body) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let (settings_fg, settings_bg) = match self.focused_panel {
            Panel::Request(RequestTab::Settings) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let tabs = [
            badge("Headers", Some(headers_fg), headers_bg),
            badge("Body", Some(body_fg), body_bg),
            badge("Settings", Some(settings_fg), settings_bg),
        ]
        .concat();

        if let Panel::Request(request_tab) = &self.focused_panel {
            match request_tab {
                RequestTab::Headers => self.render_request_headers_pane(frame, content_area),
                RequestTab::Body => self.render_request_body_pane(frame, content_area),
                RequestTab::Settings => self.render_request_settings_pane(frame, content_area),
            }
        } else {
            self.render_request_headers_pane(frame, content_area);
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
            .request_state
            .headers
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(index, (name, value))| {
                header_line(
                    name,
                    value,
                    self.request_state.current_header == index,
                    area,
                )
            })
            .collect::<Vec<_>>();

        let (add_button_fg, add_button_bg) =
            match (&self.focused_panel, &self.request_state.current_header) {
                (Panel::Request(RequestTab::Headers), RequestHeaderFocus::Add) => {
                    (palette::SUBTEXT1, palette::SURFACE1)
                }
                _ => (palette::SUBTEXT0, palette::SURFACE0),
            };

        let add_header_button =
            Line::from_iter(badge("Add Header", Some(add_button_fg), add_button_bg));

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

fn header_line<'a>(name: String, value: String, focused: bool, area: Rect) -> [Line<'a>; 2] {
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
                badge(
                    format!("{}{}", name, name_padding),
                    None,
                    if focused {
                        palette::SURFACE1
                    } else {
                        palette::SURFACE0
                    },
                ),
                vec![Span::styled(
                    ":",
                    Style::new().fg(if focused {
                        palette::TEXT
                    } else {
                        palette::SUBTEXT0
                    }),
                )],
                badge(
                    format!("{}{}", value, value_padding),
                    None,
                    if focused {
                        palette::SURFACE1
                    } else {
                        palette::SURFACE0
                    },
                ),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
        ),
        Line::from(""),
    ]
}
