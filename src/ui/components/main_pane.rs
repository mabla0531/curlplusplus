mod request_body;
mod request_headers;
mod response_body;
mod response_data;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    Application,
    state::{MainTab, Panel},
    ui::{components::badge::badge, palette},
};

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
            badge(
                "Request Headers",
                Some(request_headers_tab_fg),
                request_headers_tab_bg,
            ),
            badge(
                "Request Body",
                Some(request_body_tab_fg),
                request_body_tab_bg,
            ),
            badge(
                "Response Data",
                Some(response_data_tab_fg),
                response_data_tab_bg,
            ),
            badge(
                "Response Body",
                Some(response_body_tab_fg),
                response_body_tab_bg,
            ),
        ]
        .concat();

        match self.focused_panel {
            Panel::Main(MainTab::RequestHeaders) => {
                self.render_request_headers_pane(frame, content_area)
            }
            Panel::Main(MainTab::RequestBody) => self.render_request_body_pane(frame, content_area),
            Panel::Main(MainTab::ResponseData) => {}
            Panel::Main(MainTab::ResponseBody) => {}
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
}
