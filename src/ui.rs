pub mod animations;
pub mod components;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::Block,
};

use crate::{Application, client::WrappedResponse, state::Panel};

struct LayoutSet {
    pub app_name: Rect,
    pub method_selector: Rect,
    pub method_dropdown: Rect,
    pub url_input: Rect,
    pub main_pane: Rect,
    pub help_bar: Rect,
}

fn layout(frame: &mut Frame) -> LayoutSet {
    let page_vertical = Layout::vertical([
        Constraint::Length(3),
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
        main_pane: page_vertical[1],
        help_bar: page_vertical[2],
    }
}

impl Application {
    pub fn render(&mut self, frame: &mut Frame) {
        let layout = layout(frame);

        frame.render_widget(Block::new().bg(self.settings.theme.base), frame.area());

        self.render_title(frame, layout.app_name);
        self.render_method(frame, layout.method_selector);
        self.render_url_input(frame, layout.url_input);
        self.render_main_pane(frame, layout.main_pane);
        self.render_help_bar(frame, layout.help_bar);

        if self.editing && self.focused_panel == Panel::Method {
            // has to be after all others since it's a "z-index: 1;" element
            self.render_method_dropdown(frame, layout.method_dropdown);
        }
    }

    pub fn response_status_pane_height(&self) -> u16 {
        self.terminal_height.saturating_sub(9)
    }
}

pub fn response_status_height(terminal_width: u16, response: &WrappedResponse) -> usize {
    let headers_length = response.meta.headers.len();

    let body_length: usize = response
        .body
        .lines()
        .map(|line| {
            ((line.chars().count().max(1) as f64) / (terminal_width.saturating_sub(4) as f64))
                .ceil() as usize
        })
        .sum::<usize>()
        .max(1);

    //url/status + 1 gap + headers label + headers + 1 gap + body label + body_length
    1 + 1 + 1 + headers_length + 1 + 1 + body_length
}
