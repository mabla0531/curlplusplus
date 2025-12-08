use ratatui::{
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
    Frame,
};

use crate::{
    state::{Panel, ResponseTab},
    ui::{components::badge::badge, palette},
    Application,
};

impl Application {
    pub fn render_response_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = match self.focused_panel {
            Panel::Response(_) => Style::new().fg(palette::SKY),
            _ => Style::new().fg(palette::TEXT),
        };

        let (data_fg, data_bg) = match self.focused_panel {
            Panel::Response(ResponseTab::Data) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let (body_fg, body_bg) = match self.focused_panel {
            Panel::Response(ResponseTab::Body) => (palette::SAPPHIRE, palette::SURFACE2),
            _ => (palette::SUBTEXT0, palette::SURFACE0),
        };

        let tabs = [
            badge("Data", Some(data_fg), data_bg),
            badge("Body", Some(body_fg), body_bg),
        ]
        .concat();

        let tabs_paragraph = Paragraph::new(Line::from_iter(tabs));

        let request_data = Paragraph::new(Span::styled("goober", Style::new().fg(palette::TEXT)))
            .block(Block::new().padding(Padding::new(1, 1, 1, 1)));

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
