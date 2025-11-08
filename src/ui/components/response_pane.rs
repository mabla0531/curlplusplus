use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    Application,
    state::{Panel, ResponseTab},
    ui::{components::badge::badge, palette},
};

impl Application {
    pub fn render_response_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = if self.state.focused_panel == Panel::Response {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

        let (data_fg, data_bg) = if self.state.current_response_tab == ResponseTab::Data {
            (palette::SAPPHIRE, palette::SURFACE2)
        } else {
            (palette::SUBTEXT0, palette::SURFACE0)
        };

        let (body_fg, body_bg) = if self.state.current_response_tab == ResponseTab::Body {
            (palette::SAPPHIRE, palette::SURFACE2)
        } else {
            (palette::SUBTEXT0, palette::SURFACE0)
        };

        let tabs = [
            badge("Data", Some(data_fg), data_bg),
            badge("Body", Some(body_fg), body_bg),
        ]
        .concat();

        let tabs_paragraph = Paragraph::new(Line::from_iter(tabs));

        let request_data = Paragraph::new("goober").bg(palette::BASE);

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
            area,
        );
        frame.render_widget(tabs_paragraph, tabs_area);
        frame.render_widget(request_data, content_area);
    }
}
