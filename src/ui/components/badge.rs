use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn badge(text: &str, fg: Option<Color>, bg: Color) -> Vec<Span<'_>> {
    let mut text_style = Style::default().bg(bg);

    if let Some(fg) = fg {
        text_style = text_style.fg(fg);
    }

    vec![
        Span::styled("▐", Style::default().fg(bg)),
        Span::styled(text, text_style),
        Span::styled("▌", Style::default().fg(bg)),
    ]
}
