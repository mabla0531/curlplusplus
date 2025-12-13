mod keyboard;
mod state;
mod ui;

use crossterm::{
    cursor::SetCursorStyle,
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::{self};

use crate::state::*;

pub struct Application {
    pub focused_panel: Panel,
    pub method_state: MethodState,
    pub url_state: UrlState,
    pub request_state: RequestState,
    pub response_state: ResponseState,
    pub editing: bool,
    pub exit_request: bool,
}

impl Application {
    fn new() -> Self {
        Self {
            focused_panel: Panel::Method,
            method_state: MethodState {
                current_method: Method::Get,
                show_dropdown: false,
            },
            url_state: UrlState {
                url_input: String::new(),
            },
            request_state: RequestState {
                headers: Vec::new(),
                current_header: RequestHeaderFocus::Add,
                current_header_section: HeaderSection::Name,
                body: vec![String::new()],
                body_cursor: BodyCursor::default(),
                settings: (),
            },
            response_state: ResponseState {},
            editing: false,
            exit_request: false,
        }
    }

    fn run<T: ratatui::backend::Backend + std::io::Write>(&mut self, terminal: &mut Terminal<T>) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);

                    execute!(terminal.backend_mut(), if self.editing { SetCursorStyle::BlinkingBar } else { SetCursorStyle::SteadyBlock })?;
                }
            }

            if self.exit_request {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    fern::Dispatch::new()
        .chain(fern::log_file("session.log").unwrap())
        .apply().unwrap();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = Application::new().run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}
