use crate::{ui::components::badge::badge, ui::palette, Application};

use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

impl Application {
    pub fn render_help_bar(&self, frame: &mut Frame, area: Rect) {
        let help_spans = [
            badge("󰌒", None, palette::SURFACE0),
            vec![Span::styled("/", Style::new().fg(palette::TEXT))],
            badge("󰘶", None, palette::SURFACE0),
            badge("󰌒", None, palette::SURFACE0),
            vec![Span::styled(
                "switch pane  ",
                Style::new().fg(palette::TEXT),
            )],
            badge("󰌑", None, palette::SURFACE0),
            vec![Span::styled(
                "edit/confirm  ",
                Style::new().fg(palette::TEXT),
            )],
            badge("󱊷", None, palette::SURFACE0),
            vec![Span::styled("exit  ", Style::new().fg(palette::TEXT))],
            badge("ctrl", None, palette::SURFACE0),
            badge("󰌑", None, palette::SURFACE0),
            vec![Span::styled("send  ", Style::new().fg(palette::TEXT))],
        ]
        .concat();

        let help = Paragraph::new(Line::from_iter(help_spans)).alignment(Alignment::Right);

        frame.render_widget(help, area);
    }
}
