use ratatui::{
    layout::Rect,
    prelude::Position,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{state::Panel, Application};

impl Application {
    pub fn render_url_input(&self, frame: &mut Frame, area: Rect) {
        let border_style = match self.focused_panel {
            Panel::Url => Style::new().fg(self.settings.theme.active),
            _ => Style::new().fg(self.settings.theme.text),
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
            Span::styled(
                self.url_state.url_input.clone(),
                Style::default().fg(self.settings.theme.text),
            ),
            Span::styled(
                help_string,
                Style::default().fg(self.settings.theme.inactive_text),
            ),
        ]))
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(border_style),
        );

        if self.focused_panel == Panel::Url && self.editing {
            frame.set_cursor_position(Position::from((
                area.x + self.url_state.url_cursor as u16 + 1,
                area.y + 1,
            )));
        }
        frame.render_widget(url, area);
    }
}
