use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use std::iter;

use crate::{
    Application,
    state::{Method, Panel},
};

impl Application {
    pub fn render_method(&self, frame: &mut Frame, area: Rect) {
        let border_style = match self.focused_panel {
            Panel::Method => Style::new().fg(self.settings.theme.active),
            _ => Style::new().fg(self.settings.theme.text),
        };

        let method_str = self.method_state.current_method.to_string();
        let method_dropdown_str = format!(
            " {}{}",
            method_str,
            iter::repeat_n(' ', 9 - method_str.len()).collect::<String>()
        );

        let method = Paragraph::new(Line::from_iter(self.badge(
            method_dropdown_str,
            Some(self.settings.theme.base),
            match self.method_state.current_method {
                Method::Get => self.settings.theme.get_color,
                Method::Post => self.settings.theme.post_color,
                Method::Put => self.settings.theme.put_color,
                Method::Patch => self.settings.theme.patch_color,
                Method::Options => self.settings.theme.options_color,
                Method::Connect => self.settings.theme.connect_color,
                Method::Trace => self.settings.theme.trace_color,
                Method::Delete => self.settings.theme.delete_color,
                Method::Head => self.settings.theme.head_color,
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
