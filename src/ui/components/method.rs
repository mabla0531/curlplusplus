use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use std::iter;

use crate::{
    state::{Method, Panel},
    ui::components::badge::badge,
    ui::palette,
    Application,
};

impl Application {
    pub fn render_method(&self, frame: &mut Frame, area: Rect) {
        let border_style = match self.focused_panel {
            Panel::Method => Style::new().fg(palette::SKY),
            _ => Style::new().fg(palette::TEXT),
        };

        let method_str = self.method_state.current_method.to_string();
        let method_dropdown_str = format!(
            " {}{} ",
            method_str,
            iter::repeat_n(' ', 7 - method_str.len()).collect::<String>()
        );

        let method = Paragraph::new(Line::from_iter(badge(
            method_dropdown_str,
            Some(palette::CRUST),
            match self.method_state.current_method {
                Method::Get => palette::GET_COLOR,
                Method::Post => palette::POST_COLOR,
                Method::Put => palette::PUT_COLOR,
                Method::Patch => palette::PATCH_COLOR,
                Method::Options => palette::OPTIONS_COLOR,
                Method::Connect => palette::CONNECT_COLOR,
                Method::Trace => palette::TRACE_COLOR,
                Method::Delete => palette::DELETE_COLOR,
                Method::Head => palette::HEAD_COLOR,
            },
        )))
        .block(
            Block::new()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );
        frame.render_widget(method, area);
    }
}
