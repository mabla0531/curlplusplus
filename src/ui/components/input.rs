use crate::Application;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

impl Application {
    /// bg will be applied to every Span provided just in case
    pub fn input<'a>(&self, text: Vec<Span<'a>>, bg: Color) -> Vec<Span<'a>> {
        [
            vec![Span::styled("▐", Style::default().fg(bg))],
            text.iter()
                .cloned()
                .map(|t| {
                    let style = t.style;
                    t.style(Style {
                        bg: Some(bg),
                        ..style
                    })
                })
                .collect(),
            vec![Span::styled("▌", Style::default().fg(bg))],
        ]
        .concat()
    }
}
