mod components;
mod palette;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use std::iter;

use crate::{Application, state::FocusedPanel, ui::components::badge::badge};

struct LayoutSet {
    pub app_name: Rect,
    pub method_selector: Rect,
    pub method_dropdown: Rect,
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
        Constraint::Length(13),
        Constraint::Fill(1),
    ])
    .split(page_vertical[0]);

    let method_dropdown = Rect {
        x: top_bar[1].x + 1,
        y: top_bar[1].y + 2,
        width: 12,
        height: 11,
    };

    LayoutSet {
        app_name: top_bar[0],
        method_selector: top_bar[1],
        method_dropdown,
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

        if self.state.show_method_dropdown {
            self.render_method_dropdown(frame, layout.method_dropdown);
        }
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
        let border_style = if let FocusedPanel::Method = self.state.focused_panel {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

        let method_str = self.state.current_method.to_string();
        let method_dropdown_str = format!(
            " {}{} ",
            method_str,
            iter::repeat_n(' ', 7 - method_str.len()).collect::<String>()
        );

        let method = Paragraph::new(Line::from_iter(badge(
            method_dropdown_str.as_str(),
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
        let border_style = if self.state.focused_panel == FocusedPanel::Url {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

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

        let border_style = if self.state.focused_panel == FocusedPanel::Request {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

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

        let border_style = if self.state.focused_panel == FocusedPanel::Response {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

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

    pub fn render_method_dropdown(&self, frame: &mut Frame, area: Rect) {
        let test_text = Paragraph::new(vec![
            Line::from(Span::styled(
                "▄▄▄▄▄▄▄▄▄▄▄▄",
                Style::default().fg(palette::SURFACE0),
            )),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Get",
                    Style::default()
                        .fg(palette::GET_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Post",
                    Style::default()
                        .fg(palette::POST_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Put",
                    Style::default()
                        .fg(palette::PUT_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Patch",
                    Style::default()
                        .fg(palette::PATCH_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Options",
                    Style::default()
                        .fg(palette::OPTIONS_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Connect",
                    Style::default()
                        .fg(palette::CONNECT_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Trace",
                    Style::default()
                        .fg(palette::TRACE_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Delete",
                    Style::default()
                        .fg(palette::DELETE_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from_iter([
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
                Span::styled(
                    "Head",
                    Style::default()
                        .fg(palette::HEAD_COLOR)
                        .bg(palette::SURFACE0),
                ),
                Span::styled("██████", Style::default().fg(palette::SURFACE0)),
                Span::styled("█", Style::default().fg(palette::SURFACE0)),
            ]),
            Line::from(Span::styled(
                "▀▀▀▀▀▀▀▀▀▀▀▀",
                Style::default().fg(palette::SURFACE0),
            )),
        ]);

        frame.render_widget(test_text, area);
    }
}
