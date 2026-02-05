mod main_menu_pane;
mod main_pane;
mod method;
mod url_input;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    Application,
    state::{MainMenuSelection, MainTab, Panel},
};

impl Application {
    pub fn handle_input(&mut self, event: KeyEvent) {
        if self.show_main_menu {
            self.handle_main_menu_input(event);
        } else {
            match event.code {
                KeyCode::Char(' ') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.send_request();
                    self.focused_panel = Panel::Main(MainTab::ResponseStatus);
                }
                KeyCode::Tab if !self.editing => self.focused_panel.increment(),
                KeyCode::BackTab if !self.editing => self.focused_panel.decrement(),
                KeyCode::Esc => {
                    if self.editing {
                        self.editing = false;
                    } else {
                        self.main_menu_state.selection = MainMenuSelection::Return;
                        self.show_main_menu = true;
                    }
                }
                _ => match self.focused_panel {
                    Panel::Method => self.handle_method_input(event),
                    Panel::Url => self.handle_url_input(event),
                    Panel::Main(main_tab) => self.handle_main_pane_input(event, main_tab),
                },
            }
        }
    }
}
