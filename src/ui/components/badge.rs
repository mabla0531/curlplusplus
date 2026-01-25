use crate::Application;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

impl Application {
    pub fn badge<'a, T>(&self, text: T, fg: Option<Color>, bg: Color) -> Vec<Span<'a>>
    where
        T: Into<String>,
    {
        let mut text_style = Style::default().bg(bg);

        text_style = match fg {
            Some(fg) => text_style.fg(fg),
            None => text_style.fg(self.settings.theme.text),
        };

        let for_sure_string: String = text.into();

        vec![
            Span::styled("▐", Style::default().fg(bg)),
            Span::styled(for_sure_string, text_style),
            Span::styled("▌", Style::default().fg(bg)),
        ]
    }
}
