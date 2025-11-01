use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub fn badge(text: &str, fg: Option<(u8, u8, u8)>, bg: (u8, u8, u8)) -> Vec<Span> {
    let (bgr, bgg, bgb) = bg;

    let mut text_style = Style::default().bg(Color::Rgb(bgr, bgg, bgb));

    if let Some((fgr, fgg, fgb)) = fg {
        text_style = text_style.fg(Color::Rgb(fgr, fgg, fgb));
    }

    vec![
        Span::styled("▐", Style::default().fg(Color::Rgb(bgr, bgg, bgb))),
        Span::styled(text, text_style),
        Span::styled("▌", Style::default().fg(Color::Rgb(bgr, bgg, bgb))),
    ]
}
