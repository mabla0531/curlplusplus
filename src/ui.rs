mod components;
mod palette;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Offset, Rect},
    style::{Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{Application, state::FocusedPanel, ui::components::badge::badge};

struct LayoutSet {
    pub app_name: Rect,
    pub method_selector: Rect,
    pub url_input: Rect,
    pub request_pane: Rect,
    pub response_pane: Rect,
    pub help_bar: Rect,
}

fn layout(frame: &mut Frame) -> LayoutSet {
    let page_vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let top_bar = Layout::horizontal([
        Constraint::Length(8),
        Constraint::Length(11),
        Constraint::Fill(1),
    ])
    .split(page_vertical[0]);

    LayoutSet {
        app_name: top_bar[0],
        method_selector: top_bar[1],
        url_input: top_bar[2],
        request_pane: page_vertical[1],
        response_pane: page_vertical[2],
        help_bar: page_vertical[3],
    }
}

impl Application {
    pub fn render(&self, frame: &mut Frame) {
        let layout = layout(frame);

        frame.render_widget(Block::new().bg(palette::CRUST), frame.area());

        self.render_title(frame, layout.app_name);
        self.render_method(frame, layout.method_selector);
        self.render_url_input(frame, layout.url_input);
        self.render_request_pane(frame, layout.request_pane);
        self.render_response_pane(frame, layout.response_pane);
        self.render_help_bar(frame, layout.help_bar);
    }

    pub fn render_help_bar(&self, frame: &mut Frame, area: Rect) {
        let help_spans = [
            badge("󰌒", None, palette::SURFACE0),
            vec!["/".to_span()],
            badge("󰘶", None, palette::SURFACE0),
            badge("󰌒", None, palette::SURFACE0),
            vec!["switch pane  ".to_span()],
            badge("", None, palette::SURFACE0),
            badge("", None, palette::SURFACE0),
            badge("", None, palette::SURFACE0),
            badge("", None, palette::SURFACE0),
            vec!["interact  ".to_span()],
            badge("󰌑", None, palette::SURFACE0),
            vec!["edit  ".to_span()],
            badge("󱊷", None, palette::SURFACE0),
            vec!["exit edit  ".to_span()],
            badge("ctrl", None, palette::SURFACE0),
            badge("󰌑", None, palette::SURFACE0),
            vec!["send  ".to_span()],
            badge("q", None, palette::SURFACE0),
            vec!["quit".to_span()],
        ]
        .concat();

        let help = Paragraph::new(Line::from_iter(help_spans));

        frame.render_widget(help, area);
    }

    pub fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title_spans = [
            "curl".to_span(),
            Span::styled("++", Style::default().fg(palette::PEACH).bold()),
        ];
        let title = Paragraph::new(Line::from_iter(title_spans))
            .block(Block::new().padding(Padding::uniform(1)));

        frame.render_widget(title, area);
    }

    pub fn render_method(&self, frame: &mut Frame, area: Rect) {
        let border_style = (self.state.focused_panel == FocusedPanel::Method)
            .then_some(Style::new().fg(palette::SKY))
            .unwrap_or_default();

        let method = Paragraph::new(Line::from_iter(badge(
            "  GET  ",
            Some(palette::CRUST),
            palette::BLUE,
        )))
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );
        frame.render_widget(method, area);
    }

    pub fn render_url_input(&self, frame: &mut Frame, area: Rect) {
        let border_style = (self.state.focused_panel == FocusedPanel::Url)
            .then_some(Style::new().fg(palette::SKY))
            .unwrap_or_default();

        let url = Paragraph::new(Line::from_iter([Span::styled(
            "",
            Style::default().bg(palette::CRUST),
        )]))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

        frame.render_widget(url, area);
    }

    pub fn render_request_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = (self.state.focused_panel == FocusedPanel::Request)
            .then_some(Style::new().fg(palette::SKY))
            .unwrap_or_default();

        let tabs = [
            badge("Headers", Some(palette::SUBTEXT0), palette::SURFACE0),
            badge("Body", Some(palette::SAPPHIRE), palette::SURFACE2),
            badge("Settings", Some(palette::SUBTEXT0), palette::SURFACE0),
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

    pub fn render_response_pane(&self, frame: &mut Frame, area: Rect) {
        let sub_area = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
            .split(area.inner(Margin::new(1, 1)));

        let (tabs_area, content_area) = (sub_area[0], sub_area[1]);

        let border_style = (self.state.focused_panel == FocusedPanel::Response)
            .then_some(Style::new().fg(palette::SKY))
            .unwrap_or_default();

        let tabs = [
            badge("Data", Some(palette::SUBTEXT0), palette::SURFACE0),
            badge("Body", Some(palette::SAPPHIRE), palette::SURFACE2),
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
