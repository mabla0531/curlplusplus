use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::{Application, state::MainMenuSelection};

impl Application {
    pub fn render_main_menu_pane(&mut self, frame: &mut Frame) {
        let [_, vertical, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(10),
            Constraint::Fill(1),
        ])
        .areas(frame.area());

        let [_, layout, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(24),
            Constraint::Fill(1),
        ])
        .areas(vertical);

        let return_badge_bg = if self.main_menu_state.selection == MainMenuSelection::Return {
            self.settings.theme.active_element
        } else {
            self.settings.theme.inactive_element
        };
        let settings_badge_bg = if self.main_menu_state.selection == MainMenuSelection::Settings {
            self.settings.theme.active_element
        } else {
            self.settings.theme.inactive_element
        };
        let exit_badge_bg = if self.main_menu_state.selection == MainMenuSelection::Exit {
            self.settings.theme.active_element
        } else {
            self.settings.theme.inactive_element
        };

        let return_badge = Line::from_iter(
            [
                vec![Span::styled(
                    "█████",
                    Style::default().fg(self.settings.theme.base),
                )],
                self.badge("  Return  ", None, return_badge_bg),
                vec![Span::styled(
                    "█████",
                    Style::default().fg(self.settings.theme.base),
                )],
            ]
            .concat(),
        );
        let settings_badge = Line::from_iter(
            [
                vec![Span::styled(
                    "█████",
                    Style::default().fg(self.settings.theme.base),
                )],
                self.badge(" Settings ", None, settings_badge_bg),
                vec![Span::styled(
                    "█████",
                    Style::default().fg(self.settings.theme.base),
                )],
            ]
            .concat(),
        );
        let exit_badge = Line::from_iter(
            [
                vec![Span::styled(
                    "█████",
                    Style::default().fg(self.settings.theme.base),
                )],
                self.badge("   Exit   ", None, exit_badge_bg),
                vec![Span::styled(
                    "█████",
                    Style::default().fg(self.settings.theme.base),
                )],
            ]
            .concat(),
        );

        let menu = Paragraph::new(Text::from_iter([
            Line::from("██████████████████████")
                .style(Style::default().fg(self.settings.theme.base)),
            return_badge,
            Line::from("██████████████████████")
                .style(Style::default().fg(self.settings.theme.base)),
            settings_badge,
            Line::from("██████████████████████")
                .style(Style::default().fg(self.settings.theme.base)),
            exit_badge,
            Line::from("██████████████████████")
                .style(Style::default().fg(self.settings.theme.base)),
            Line::from_iter([
                Span::styled(
                    "         v1.0.0 2/6/26",
                    Style::default()
                        .fg(self.settings.theme.inactive_element)
                        .bg(self.settings.theme.base),
                ),
                Span::styled("█████████", Style::default().fg(self.settings.theme.base)),
            ]),
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().bg(self.settings.theme.base));

        frame.render_widget(menu, layout);
    }
}
