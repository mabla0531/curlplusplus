mod keyboard;
mod state;
mod ui;

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self};

use crate::state::{
    AppState, FocusedRequestHeadersElement, Method, Panel, RequestTab, ResponseTab,
};

pub struct Application {
    pub state: AppState,
    pub exit_request: bool,
}

impl Application {
    fn new() -> Self {
        Self {
            state: AppState {
                focused_panel: Panel::Method,

                current_method: Method::Get,
                show_method_dropdown: false,

                url_input: String::new(),

                current_request_tab: RequestTab::Headers,
                request_headers: vec![
                    ("hi".to_string(), "goobr".to_string()),
                    ("header2".to_string(), "bonk".to_string()),
                ],
                request_body: String::new(),
                request_settings: (),
                focused_element: FocusedRequestHeadersElement::Tabs,

                current_response_tab: ResponseTab::Data,
            },
            exit_request: false,
        }
    }

    fn run<T: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<T>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                }
            }

            if self.exit_request {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = Application::new().run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}
