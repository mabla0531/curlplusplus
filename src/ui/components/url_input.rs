use ratatui::{
    Frame,
    layout::Rect,
    prelude::Position,
    style::Style,
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{Application, defs::URL_AUTOCOMPLETES, state::Panel};

impl Application {
    pub fn render_url_input(&self, frame: &mut Frame, area: Rect) {
        let border_style = match self.focused_panel {
            Panel::Url => Style::new().fg(self.settings.theme.active),
            _ => Style::new().fg(self.settings.theme.text),
        };

        let autocomplete_candidate = URL_AUTOCOMPLETES
            .iter()
            .find_map(|(k, v)| self.url_state.url_input.eq(k).then_some(*v));

        let help_string = if let Some(autocomplete_candidate) = autocomplete_candidate
            && self.focused_panel == Panel::Url
        {
            format!("{} 󰛂", autocomplete_candidate)
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
