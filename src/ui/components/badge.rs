use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn badge<'a, T>(text: T, fg: Option<Color>, bg: Color) -> Vec<Span<'a>>
where
    T: Into<String>,
{
    let mut text_style = Style::default().bg(bg);

    if let Some(fg) = fg {
        text_style = text_style.fg(fg);
    }

    let for_sure_string: String = text.into();

    vec![
        Span::styled("▐", Style::default().fg(bg)),
        Span::styled(for_sure_string, text_style),
        Span::styled("▌", Style::default().fg(bg)),
    ]
}
