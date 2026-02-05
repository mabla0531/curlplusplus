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
        let (tab_key, shift_key, enter_key, escape_key, control_key, space_key) =
            match self.settings.symbols {
                true => ("󰌒", "󰘶", "󰌑", "󱊷", "󰘴", "󱁐"),
                false => ("tab", "shift", "enter", "esc", "ctrl", "space"),
            };

        let help_spans = [
            self.badge(tab_key, None, self.settings.theme.inactive_element),
            vec![Span::styled("/", Style::new().fg(self.settings.theme.text))],
            self.badge(shift_key, None, self.settings.theme.inactive_element),
            self.badge(tab_key, None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "switch pane  ",
                Style::new().fg(self.settings.theme.text),
            )],
            self.badge(enter_key, None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "edit/confirm  ",
                Style::new().fg(self.settings.theme.text),
            )],
            self.badge(escape_key, None, self.settings.theme.inactive_element),
            vec![Span::styled(
                "show menu  ",
                Style::new().fg(self.settings.theme.text),
            )],
            self.badge(control_key, None, self.settings.theme.inactive_element),
            self.badge(space_key, None, self.settings.theme.inactive_element),
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
