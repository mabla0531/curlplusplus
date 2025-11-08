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
        let border_style = if self.state.focused_panel == Panel::Url {
            Style::new().fg(palette::SKY)
        } else {
            Default::default()
        };

        let url = Paragraph::new(Line::from_iter([Span::styled(
            self.state.url_input.clone(),
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
}
