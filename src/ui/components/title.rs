use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Padding, Paragraph},
    Frame,
};

use crate::Application;

impl Application {
    pub fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title_spans = [
            Span::styled("curl", Style::default().fg(self.settings.theme.text)),
            Span::styled("++", Style::default().fg(self.settings.theme.accent).bold()),
        ];
        let title = Paragraph::new(Line::from_iter(title_spans))
            .block(Block::new().padding(Padding::uniform(1)));

        frame.render_widget(title, area);
    }
}
