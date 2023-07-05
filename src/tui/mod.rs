mod display;
mod menu;
mod raw_mode_guard;
mod util;

use crossterm::{
    self,
    cursor::MoveToColumn,
    event::{DisableFocusChange, Event, KeyCode, KeyEventKind, poll, read},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, SetTitle},
};
use std::{
    cmp::{min, max},
    io::{self, Stdout, stdout, Write},
    time::Duration
};
use crate::error::{ConeRoboError, Result};
use display::*;
use menu::*;
use raw_mode_guard::*;

const NUM_MENUS: usize = 2;

pub fn launch_user_interface() -> Result<()> {
    log::info!("Enabling raw mode");
    let _raw_mode_guard = RawModeGuard::new()?;
    
    // `Menu.state` must be unique for every `Menu` initialized.
    let menus = init_menus();
    let mut curr_menu = menus[0].clone();
    let mut option_index = 0i32;

    log::info!("Configuring terminal");
    let mut stdout = stdout();
    configure_terminal(&mut stdout)
        .map_err(ConeRoboError::I0000)?;

    log::info!("Entering main loop of TUI");
    let mut msg_queue = MessageQueue::new();
    loop {
        display_menu(&mut stdout, &curr_menu, &option_index, &mut msg_queue)
            .map_err(ConeRoboError::I0000)?;
        let quit_attempt = process_events(
            &menus, &mut curr_menu, &mut option_index, &mut msg_queue
        )?;
        if quit_attempt {
            break
        }
    }

    log::info!("Quit attempt received. Terminating TUI...");
    Ok(())
}

// The number of menus should (for the time being), be known at compile time.
fn init_menus() -> [Menu; NUM_MENUS] {
    [
        Menu {
            title: "Main Menu",
            state: MenuState(0),
            options: &[
                MenuOption {
                    option_str: "Component Menu",
                    action: Action::Navigation { next_menu: MenuState(1) }
                },
                MenuOption {
                    option_str: "Exit TUI",
                    action: Action::QuitAttempt
                }
            ]
        },
        Menu {
            title: "Component Menu",
            state: MenuState(1),
            options: &[
                MenuOption {
                    option_str: "Load Components",
                    action: Action::Unavailable
                },
                MenuOption {
                    option_str: "Manage Loaded Components",
                    action: Action::Unavailable
                },
                MenuOption {
                    option_str: "Back to Main Menu",
                    action: Action::Navigation { next_menu: MenuState(0) }
                }
            ]
        }
    ]
}

fn configure_terminal(stdout: &mut Stdout) -> io::Result<()> {
    let title = "ConeRobo TUI";
    crossterm::execute!(
        stdout,
        DisableFocusChange,
        SetTitle(title)
    )?;
    Ok(())
}

fn display_menu(
    stdout: &mut Stdout, curr_menu: &Menu, option_index: &i32, msg_queue: &mut MessageQueue
) -> io::Result<()> {
    log::info!("Displaying menu");

    // Top border
    let top_title = Title::Is(curr_menu.title.to_owned());
    let top_border = create_border('=', top_title).unwrap();
    crossterm::queue!(
        stdout,
        Clear(ClearType::All),
        MoveToColumn(0),
        SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
        Print(top_border + "\n"),
        ResetColor
    )?;

    // Options
    crossterm::queue!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
        Print("Options:\n"),
        ResetColor
    )?;
    for (i, menu_option) in curr_menu.options.iter().enumerate() {
        if i == *option_index as usize {
            crossterm::queue!(
                stdout,
                SetForegroundColor(Color::Rgb { r: 224, g: 210, b: 58 }),
                Print("\t["),
                SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
                Print(menu_option.option_str),
                SetForegroundColor(Color::Rgb { r: 224, g: 210, b: 58 }),
                Print("]\n"),
                ResetColor
            )?;
        } else {
            crossterm::queue!(
                stdout,
                SetForegroundColor(Color::Rgb { r: 50, g: 50, b: 50 }),
                Print(format!("\t {}\n", menu_option.option_str)),
                ResetColor
            )?;
        }
    }

    // Bottom/Message borders
    let message_border = create_border('-', Title::Empty).unwrap();
    let bottom_border = create_border('=', Title::Empty).unwrap();
    crossterm::queue!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
        Print(message_border + "\n"),
        Print(msg_queue.pop_msg() + "\n"),
        Print(bottom_border),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}

// Returns true if a quit attempt was processed.
// Potential errors include I0000, I0001, I0002.
fn process_events(
    menus: &[Menu], curr_menu: &mut Menu, option_index: &mut i32, msg_queue: &mut MessageQueue
) -> Result<bool> {
    log::info!("Waiting for event...");
    match read().map_err(ConeRoboError::I0000)? {
        Event::Key(event) => {
            log::info!("Reading key event");
            match event.kind {
                KeyEventKind::Press => {},
                KeyEventKind::Repeat => {},
                KeyEventKind::Release => {
                    match event.code {
                        KeyCode::Up => {
                            log::debug!("Up key released");
                            *option_index = max(0, *option_index - 1);
                        },
                        KeyCode::Down => {
                            log::debug!("Down key released");
                            *option_index = min(curr_menu.options.len() as i32 - 1,
                                                *option_index + 1)
                        },
                        KeyCode::Enter => {
                            log::debug!("Enter key released");
                            match curr_menu.options.get(*option_index as usize) {
                                Some(menu_option) => {
                                    match menu_option.action {
                                        Action::Unavailable => {
                                            msg_queue.push_msg("Option unavailable".to_owned())
                                        },
                                        Action::QuitAttempt => return Ok(true),
                                        Action::Navigation { next_menu } => {
                                            *curr_menu = get_next_menu(menus, next_menu)?;
                                            *option_index = 0;
                                        }
                                    }
                                }
                                None => {
                                    return Err(ConeRoboError::I0001(option_index.to_string()))
                                }
                            }
                        },
                        _ => {}
                    }
                }
            }
        },
        Event::Resize(width, height) => {
            log::info!("Reading resize event");
            let (original_size, new_size) = flush_resize_events((width, height));
            log::debug!("Resized from: {:?}, to: {:?}\r", original_size, new_size);
        },
        Event::FocusGained => {},
        Event::FocusLost => {},
        Event::Mouse(_event) => {},
        Event::Paste(_event) => {}
    }

    Ok(false)
}

fn get_next_menu(menus: &[Menu], next_menu: MenuState) -> Result<Menu> {
    for menu in menus {
        if menu.state == next_menu {
            return Ok(menu.clone())
        }
    };

    Err(ConeRoboError::I0002(next_menu.0.to_string()))
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