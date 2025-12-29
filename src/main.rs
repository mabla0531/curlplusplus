mod keyboard;
mod state;
mod ui;

use chrono::prelude::Utc;
use crossterm::{
    cursor::SetCursorStyle,
    event::{self, EnableBracketedPaste, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use ropey::Rope;
use std::io::{self};

use crate::state::*;

pub struct Application {
    pub focused_panel: Panel,
    pub method_state: MethodState,
    pub url_state: UrlState,
    pub main_state: MainState,
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
                url_cursor: 0,
            },
            main_state: MainState {
                headers: Vec::new(),
                current_header: RequestHeaderFocus::Add,
                current_header_section: HeaderSection::Name,
                current_header_cursor: 0,
                request_body: Rope::new(),
                request_body_cursor: BodyCursor {
                    position: 0,
                    target_character: 0,
                },
            },
            editing: false,
            exit_request: false,
        }
    }

    fn run<T: ratatui::backend::Backend + std::io::Write>(
        &mut self,
        terminal: &mut Terminal<T>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    self.handle_input(key);
                    execute!(
                        terminal.backend_mut(),
                        if self.editing {
                            SetCursorStyle::BlinkingBar
                        } else {
                            SetCursorStyle::SteadyBlock
                        }
                    )?;
                } else if let Event::Paste(text) = event::read()? {
                    self.handle_paste(text);
                }
            }

            if self.exit_request {
                return Ok(());
            }
        }
    }
}

fn main() -> io::Result<()> {
    let log_folder = format!(
        "{}/logs/",
        std::env::var("$HOME").unwrap_or(".".to_string())
    );

    std::fs::create_dir_all(&log_folder)
        .unwrap_or_else(|_| panic!("Could not create log folder in {}", log_folder));

    fern::Dispatch::new()
        .chain(
            fern::log_file(format!(
                "./logs/{}.log",
                Utc::now().naive_utc().format("%Y%m%d_%H%M%S")
            ))
            .unwrap(),
        )
        .apply()
        .unwrap();

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    execute!(stdout, EnableBracketedPaste)?;

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
