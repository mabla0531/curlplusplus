use crossterm::event::{KeyCode, KeyEvent};

use crate::{Application, state::MainMenuSelection};

impl Application {
    pub fn handle_main_menu_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => self.show_main_menu = false,
            KeyCode::Up => self.main_menu_state.selection.decrement(),
            KeyCode::Down => self.main_menu_state.selection.increment(),
            KeyCode::Enter => match self.main_menu_state.selection {
                MainMenuSelection::Return => self.show_main_menu = false,
                MainMenuSelection::Settings => {}
                MainMenuSelection::Exit => self.exit_request = true,
            },
            _ => {}
        }
    }
}
