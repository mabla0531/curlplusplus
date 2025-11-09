use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    Application,
    state::{Panel, RequestTab},
    ui::{components::badge::badge, palette},
};

impl Application {
    pub fn render_request_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
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

        let tabs_paragraph = Paragraph::new(Line::from_iter(tabs));

        let test_header_1 = Paragraph::new("my_header_1").block(Block::new());
        let test_header_2 = Paragraph::new("my_header_2").block(Block::new());
        let test_header_3 = Paragraph::new("my_header_3").block(Block::new());
        let add_header_button = Paragraph::new(Line::from_iter(badge(
            "Add Header",
            Some(palette::SUBTEXT0),
            palette::SURFACE0,
        )));

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
            area,
        );
        frame.render_widget(tabs_paragraph, tabs_area);
        frame.render_widget(add_header_button, content_area);
    }
}
