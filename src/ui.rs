mod components;
mod palette;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{Application, ui::components::badge::badge};

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

        self.render_title(frame, layout.app_name);
        self.render_method(frame, layout.method_selector);
        self.render_url_input(frame, layout.url_input);
        self.render_request_pane(frame, layout.request_pane);
        self.render_response_pane(frame, layout.response_pane);
        self.render_help_bar(frame, layout.help_bar);
    }

    pub fn render_help_bar(&self, frame: &mut Frame, area: Rect) {
        let mut help_spans = badge("q", None, palette::SURFACE0);
        help_spans.push("quit ".to_span());
        help_spans.append(&mut badge("󰌑", None, palette::SURFACE0));
        help_spans.push("edit ".to_span());
        help_spans.append(&mut badge("ctrl", None, palette::SURFACE0));
        help_spans.push("+".to_span());
        help_spans.append(&mut badge("󰌑", None, palette::SURFACE0));
        help_spans.push("send".to_span());

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
        let method = Paragraph::new(Line::from_iter(badge(
            "  GET  ",
            Some(palette::CRUST),
            palette::BLUE,
        )))
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Rounded),
        );
        frame.render_widget(method, area);
    }

    pub fn render_url_input(&self, frame: &mut Frame, area: Rect) {
        let url = Paragraph::new(Line::from_iter([Span::styled(
            "",
            Style::default().bg(palette::CRUST),
        )]))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(url, area);
    }

    pub fn render_request_pane(&self, frame: &mut Frame, area: Rect) {
        let tabs = [
            badge("Headers", None, palette::SURFACE0),
            badge("Body", None, palette::SURFACE1),
            badge("Settings", None, palette::SURFACE0),
        ]
        .into_iter()
        .flatten();
        let tabs_paragraph = Paragraph::new(Line::from_iter(tabs)).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(tabs_paragraph, area);
    }

    pub fn render_response_pane(&self, frame: &mut Frame, area: Rect) {
        let tabs = [
            badge("Data", None, palette::SURFACE1),
            badge("Body", None, palette::SURFACE0),
        ]
        .into_iter()
        .flatten();
        let tabs_paragraph = Paragraph::new(Line::from_iter(tabs)).block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );

        frame.render_widget(tabs_paragraph, area);
    }
}
