use crossterm::event::poll;
use crossterm::{
    self,
    event::{
        read, Event, KeyCode,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    io::stdout,
    time::Duration,
};

#[derive(Clone)]
enum MenuState {
    MainMenu,
    GUILaunch,
    ComponentLaunch,
}

pub fn launch_debug_interface() -> crossterm::Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(stdout)?;

    launch_tui();

    execute!(stdout)?;

    disable_raw_mode()
}

fn launch_tui() {
    let mut menu_state = MenuState::MainMenu;
    loop {
        render_menu(&menu_state);
        
        match process_events(&mut menu_state) {
            Ok(quit_attempt) => {
                if quit_attempt { break }
            },
            Err(err) => println!("Error: {:?}\r", err)
        }
    }
}

fn render_menu(state: &MenuState) {
    match state {
        MenuState::MainMenu => {
            print!("\
============ ConeRobo Custom Launch ============
Options:
    0 > Configure GUI Launch
    1 > Configure component management launch
    2 > Launch individual core module
    3 > Launch entire core
    Q > Exit TUI
------------------------------------------------
> ");
        },
        MenuState::GUILaunch => {
            print!("\
============= Customize GUI Launch =============
Options:
    Q > Back to main menu
------------------------------------------------
> ");
        },
        MenuState::ComponentLaunch => {
            print!("\
========== Customize Component Launch ==========
Options:
    Q > Back to main menu
------------------------------------------------
> ");
        }
    }
}

// Returns true if a quit attempt was processed.
fn process_events(state: &mut MenuState) -> crossterm::Result<bool> {
    // Will block until an event is received.
    match read()? {
        Event::Key(event) => {
            match state {
                MenuState::MainMenu => {
                    match event.code {
                        KeyCode::Char('0') => *state = MenuState::GUILaunch,
                        KeyCode::Char('1') => *state = MenuState::ComponentLaunch,
                        KeyCode::Char('2') => println!("Option unavailable"),
                        KeyCode::Char('3') => println!("Option unavailable"),
                        KeyCode::Char('q') => return Ok(true),
                        _ => println!("Unknown command: '{:?}\r'", event)
                    }
                },
                MenuState::GUILaunch => {
                    match event.code {
                        KeyCode::Char('q') => *state = MenuState::MainMenu,
                        _ => println!("Unknown command: '{:?}\r'", event)
                    }
                },
                MenuState::ComponentLaunch => {
                    match event.code {
                        KeyCode::Char('q') => *state = MenuState::MainMenu,
                        _ => println!("Unknown command: '{:?}\r'", event)
                    }
                }
            }
        },
        Event::Resize(width, height) => {
            let (original_size, new_size) = flush_resize_events((width, height));
            println!("Resize from: {:?}, to: {:?}\r", original_size, new_size);
        },
        Event::FocusGained => {},
        Event::FocusLost => {},
        Event::Mouse(_event) => {},
        Event::Paste(_event) => {}
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

    return (first_resize, last_resize);
}