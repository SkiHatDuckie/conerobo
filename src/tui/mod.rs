mod app;
mod ui;

use crate::{
    error::{ConeRoboError, Result},
    tui::app::App,
};
use crossterm::{
    self,
    event::{DisableFocusChange, Event, KeyCode, KeyEventKind, poll, read},
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen,
        SetTitle},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io::{self, Stdout, stdout},
    time::Duration,
};

pub fn launch_user_interface() -> Result<()> {
    enable_raw_mode().map_err(ConeRoboError::I0000)?;

    let mut terminal = setup_terminal()
        .map_err(ConeRoboError::I0000)?;

    let app = App::new();
    run_app(&mut terminal, app)
        .map_err(ConeRoboError::I0000)?;

    log::info!("Restoring terminal");
    disable_raw_mode().map_err(ConeRoboError::I0000)?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    ).map_err(ConeRoboError::I0000)?;

    Ok(())
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    log::info!("Setting up terminal");
    let mut stdout = stdout();
    crossterm::execute!(
        stdout,
        DisableFocusChange,
        EnterAlternateScreen,
        SetTitle("ConeRobo TUI"),
    )?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    log::info!("Running app in terminal");
    let quit_attempt = true;
    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;
        if process_events(&mut app)? == quit_attempt {
            log::info!("Quit attempt received. Terminating app");
            return Ok(())
        };
    }
}

// Returns `true` if a quit attempt is received.
fn process_events(app: &mut App) -> io::Result<bool> {
    match read()? {
        Event::Key(event) => {
            match event.kind {
                KeyEventKind::Release => {
                    log::debug!("Received key event: {:?}", event.code);
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
            let (original_size, new_size) = flush_resize_events((width, height));
            log::debug!("Resized from: {:?}, to: {:?}", original_size, new_size);
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