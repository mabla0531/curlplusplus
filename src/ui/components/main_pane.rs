mod request_body;
mod request_headers;
mod response_status;

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{
    Application,
    state::{MainTab, Panel},
};

impl Application {
    pub fn render_main_pane(&mut self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);
        let tabs_area =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(tabs_area);
        let (request_tabs_area, response_tabs_area) = (tabs_area[0], tabs_area[1]);

        let border_style = match self.focused_panel {
            Panel::Main(_) => Style::new().fg(self.settings.theme.active),
            _ => Style::new().fg(self.settings.theme.text),
        };

        let (request_headers_tab_fg, request_headers_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::RequestHeaders) => (
                self.settings.theme.active_text,
                self.settings.theme.active_element,
            ),
            _ => (
                self.settings.theme.inactive_text,
                self.settings.theme.inactive_element,
            ),
        };

        let (request_body_tab_fg, request_body_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::RequestBody) => (
                self.settings.theme.active_text,
                self.settings.theme.active_element,
            ),
            _ => (
                self.settings.theme.inactive_text,
                self.settings.theme.inactive_element,
            ),
        };

        let (response_status_tab_fg, response_status_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::ResponseStatus) => (
                self.settings.theme.active_text,
                self.settings.theme.active_element,
            ),
            _ => (
                self.settings.theme.inactive_text,
                self.settings.theme.inactive_element,
            ),
        };

        let (response_body_tab_fg, response_body_tab_bg) = match self.focused_panel {
            Panel::Main(MainTab::ResponseStatus) => (
                self.settings.theme.active_text,
                self.settings.theme.active_element,
            ),
            _ => (
                self.settings.theme.inactive_text,
                self.settings.theme.inactive_element,
            ),
        };

        let request_tabs = [
            self.badge(
                "Request Headers",
                Some(request_headers_tab_fg),
                request_headers_tab_bg,
            ),
            self.badge(
                "Request Body",
                Some(request_body_tab_fg),
                request_body_tab_bg,
            ),
        ]
        .concat();

        let response_tabs = [self.badge(
            "Response Status",
            Some(response_status_tab_fg),
            response_status_tab_bg,
        )]
        .concat();

        match self.focused_panel {
            Panel::Main(MainTab::RequestHeaders) => {
                self.render_request_headers_pane(frame, content_area)
            }
            Panel::Main(MainTab::RequestBody) => self.render_request_body_pane(frame, content_area),
            Panel::Main(MainTab::ResponseStatus) => {
                self.render_response_status_pane(frame, content_area);
            }
            _ => {}
        }

        let request_tabs_paragraph = Paragraph::new(Line::from_iter(request_tabs));
        let response_tabs_paragraph =
            Paragraph::new(Line::from_iter(response_tabs)).alignment(Alignment::Right);

        frame.render_widget(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
            area,
        );
        frame.render_widget(request_tabs_paragraph, request_tabs_area);
        frame.render_widget(response_tabs_paragraph, response_tabs_area);
    }
}
