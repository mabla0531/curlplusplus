use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{Application, state::Panel, ui::palette};

impl Application {
    pub fn render_url_input(&self, frame: &mut Frame, area: Rect) {
        let border_style = match self.focused_panel {
            Panel::Url => Style::new().fg(palette::SKY),
            _ => Style::new().fg(palette::TEXT),
        };

        let help_string = if self.focused_panel != Panel::Url {
            String::new()
        } else if self.url_state.url_input == "https" || self.url_state.url_input == "http" {
            ":// 󰛂".to_string()
        } else if "htt".starts_with(self.url_state.url_input.as_str())
            || self.url_state.url_input.is_empty()
        {
            "http 󰛂".replace(self.url_state.url_input.as_str(), "")
        } else if self.url_state.url_input.ends_with(".") {
            "com 󰛂".to_string()
        } else {
            String::new()
        };

        let url = Paragraph::new(Line::from_iter([
            Span::styled(self.url_state.url_input.clone(), Style::default()),
            Span::styled(help_string, Style::default().fg(palette::SUBTEXT0)),
        ]))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

        frame.render_widget(url, area);
    }
}
