mod ui;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::Paragraph,
};
use std::{
    io::{self},
    rc::Rc,
};

use ui::components::badge::badge;

#[derive(Clone, Debug, Default, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing,
}

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn layout(frame: &mut Frame) -> Rc<[Rect]> {
    let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).split(frame.area());
    layout
}

fn run<T: ratatui::backend::Backend>(terminal: &mut Terminal<T>) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let layout = layout(frame);

            let title_spans = [
                "curl".to_span(),
                Span::styled("++", Style::default().fg(Color::Rgb(212, 175, 55)).bold()),
            ];
            let paragraph = Paragraph::new(Line::from_iter(title_spans));

            let mut help_spans = badge("q", None, (75, 75, 75));
            help_spans.push("quit ".to_span());
            help_spans.append(&mut badge("󰌑", None, (75, 75, 75)));
            help_spans.push("edit".to_span());

            let help = Paragraph::new(Line::from_iter(help_spans));

            frame.render_widget(paragraph, layout[0]);
            frame.render_widget(help, layout[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
