use crate::Application;

use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
};

impl Application {
    pub fn render_help_bar(&self, frame: &mut Frame, area: Rect) {
        let keys = match self.settings.symbols {
            true => ["󰌒", "󰘶", "󰌒", "󰌑", "󱊷", "ctrl", "󰌑"],
            false => ["tab", "shift", "tab", "enter", "esc", "ctrl", "enter"],
        };

        let help_spans = [
            self.badge(keys[0], None, self.settings.theme.inactive_element),
            vec![Span::styled("/", Style::new().fg(self.settings.theme.text))],
            self.badge(keys[1], None, self.settings.theme.inactive_element),
            self.badge(keys[2], None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "switch pane  ",
                Style::new().fg(self.settings.theme.text),
            )],
            self.badge(keys[3], None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "edit/confirm  ",
                Style::new().fg(self.settings.theme.text),
            )],
            self.badge(keys[4], None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "exit  ",
                Style::new().fg(self.settings.theme.text),
            )],
            self.badge(keys[5], None, self.settings.theme.inactive_element),
            self.badge(keys[6], None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "send  ",
                Style::new().fg(self.settings.theme.text),
            )],
        ]
        .concat();

        let help = Paragraph::new(Line::from_iter(help_spans)).alignment(Alignment::Right);

        frame.render_widget(help, area);
    }
}
