use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, Padding, Paragraph},
};

use crate::Application;

use crate::ui::palette;

impl Application {
    pub fn render_title(&self, frame: &mut Frame, area: Rect) {
        let title_spans = [
            "curl".to_span(),
            Span::styled("++", Style::default().fg(palette::PEACH).bold()),
        ];
        let title = Paragraph::new(Line::from_iter(title_spans))
            .block(Block::new().padding(Padding::uniform(1)));

        frame.render_widget(title, area);
    }
}
