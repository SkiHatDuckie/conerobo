mod raw_mode_guard;

use crossterm::{
    self,
    event::{DisableFocusChange, Event, KeyCode, KeyEventKind, poll, read},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame, Terminal
};
use std::{
    io::{self, Stdout, stdout},
    time::Duration
};
use crate::{
    error::{ConeRoboError, Result},
    tui::raw_mode_guard::*
};

struct AppTUI<'a> {
    tab_titles: Vec<&'a str>,
    tab_index: usize
}
impl<'a> AppTUI<'a> {
    fn new() -> AppTUI<'a> {
        AppTUI {
            tab_titles: vec!["Home", "Components", "Tracker"],
            tab_index: 0,
        }
    }
    pub fn next(&mut self) {
        self.tab_index = (self.tab_index + 1) % self.tab_titles.len();
    }
    pub fn previous(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = self.tab_titles.len() - 1;
        }
    }
}

pub fn launch_user_interface() -> Result<()> {
    log::info!("Enabling raw mode");
    let _raw_mode_guard = RawModeGuard::new()?;

    log::info!("Setting up terminal");
    let mut terminal = setup_terminal()
        .map_err(ConeRoboError::I0000)?;

    log::info!("Instantiating TUI");
    let app = AppTUI::new();
    run_app(&mut terminal, app)
        .map_err(ConeRoboError::I0000)?;

    log::info!("Restoring terminal");
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    ).map_err(ConeRoboError::I0000)?;

    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = stdout();
    crossterm::execute!(
        stdout,
        DisableFocusChange,
        EnterAlternateScreen,
        SetTitle("ConeRobo TUI")
    )?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

const QUIT_ATTEMPT: bool = true;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: AppTUI) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, &app))?;
        if process_events(&mut app)? == QUIT_ATTEMPT {
            return Ok(())
        };
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &AppTUI) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0)
            ].as_ref()
        )
        .split(size);
    let block = Block::default()
        .style(Style::default()
            .bg(Color::White)
            .fg(Color::Black));
    frame.render_widget(block, size);
    let tab_titles = app
        .tab_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
            ])
        })
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Tabs"))
        .select(app.tab_index)
        .style(Style::default()
            .fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black));
    frame.render_widget(tabs, chunks[0]);
    let inner = match app.tab_index {
        0 => Block::default().title("ConeRobo Homepage").borders(Borders::ALL),
        1 => Block::default().title("Components").borders(Borders::ALL),
        2 => Block::default().title("Tracker").borders(Borders::ALL),
        _ => unreachable!()
    };
    frame.render_widget(inner, chunks[1]);
}

// Returns `true` if a quit attempt is received.
fn process_events(app: &mut AppTUI) -> io::Result<bool> {
    log::info!("Waiting for event...");
    match read()? {
        Event::Key(event) => {
            log::info!("Reading key event");
            match event.kind {
                KeyEventKind::Release => {
                    match event.code {
                        KeyCode::Char('q') => return Ok(true),
                        KeyCode::Right => app.next(),
                        KeyCode::Left => app.previous(),
                        _ => {}
                    }
                }
                _ => {}
            }
        },
        Event::Resize(width, height) => {
            log::info!("Reading resize event");
            let (original_size, new_size) = flush_resize_events((width, height));
            log::debug!("Resized from: {:?}, to: {:?}\r", original_size, new_size);
        },
        _ => {}
    }
    Ok(false)
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
    let mut last_resize = first_resize;
    while let Ok(true) = poll(Duration::from_millis(50)) {
        if let Ok(Event::Resize(x, y)) = read() {
            last_resize = (x, y);
        }
    }

    (first_resize, last_resize)
}