use crate::{Application, ui::components::badge::badge, ui::palette};

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    text::{Line, ToSpan},
    widgets::Paragraph,
};

impl Application {
    pub fn render_help_bar(&self, frame: &mut Frame, area: Rect) {
        let help_spans = [
            badge("󰌒", None, palette::SURFACE0),
            vec!["/".to_span()],
            badge("󰘶", None, palette::SURFACE0),
            badge("󰌒", None, palette::SURFACE0),
            vec!["switch pane  ".to_span()],
            badge("󰌑", None, palette::SURFACE0),
            vec!["edit/confirm  ".to_span()],
            badge("󱊷", None, palette::SURFACE0),
            vec!["exit  ".to_span()],
            badge("ctrl", None, palette::SURFACE0),
            badge("󰌑", None, palette::SURFACE0),
            vec!["send  ".to_span()],
        ]
        .concat();

        let help = Paragraph::new(Line::from_iter(help_spans)).alignment(Alignment::Right);

        frame.render_widget(help, area);
    }
}
