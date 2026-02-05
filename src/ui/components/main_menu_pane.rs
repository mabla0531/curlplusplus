use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{Application, state::MainMenuSelection};

impl Application {
    pub fn render_main_menu_pane(&mut self, frame: &mut Frame) {
        let [_, vertical, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(9),
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

        let return_badge = Line::from_iter(self.badge("  Return  ", None, return_badge_bg));
        let settings_badge = Line::from_iter(self.badge(" Settings ", None, settings_badge_bg));
        let exit_badge = Line::from_iter(self.badge("   Exit   ", None, exit_badge_bg));

        let menu = Paragraph::new(Text::from_iter([
            return_badge,
            Line::from(" "),
            settings_badge,
            Line::from(" "),
            exit_badge,
        ]))
        .alignment(Alignment::Center)
        .block(
            Block::new()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::uniform(1)),
        );
        frame.render_widget(menu, layout);
    }
}
