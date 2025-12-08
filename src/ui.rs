mod components;
mod palette;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Block,
};

use crate::{state::Panel, Application};

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

        if self.editing && self.focused_panel == Panel::Method { // has to be after all others since it's a "z-index: 1;" element
            self.render_method_dropdown(frame, layout.method_dropdown);
        }
    }
}
