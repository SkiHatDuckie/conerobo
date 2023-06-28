use crossterm::{
    self,
    event::{Event, KeyCode, KeyEventKind, poll, read},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, SetTitle},
};
use std::{
    cmp::{min, max},
    io::{self, Stdout, stdout, Write},
    time::Duration
};

use crate::error::Result;
mod menu;
use menu::*;
mod raw_mode_guard;
use raw_mode_guard::*;
mod util;
use util::get_terminal_width;

// `Menu.state` must be unique for every `Menu` initialized.
const MENUS: &[Menu] = &[
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
                option_str: "Back to Main Menu",
                action: Action::Navigation { next_menu: MenuState(0) }
            },
        ]
    }
];

pub fn launch_user_interface() -> Result<()> {
    log::info!("Enabling raw mode...");
    let _raw_mode_guard = RawModeGuard::new()?;
    log::info!("Raw mode enabled");

    let mut stdout = stdout();
    let mut curr_menu = MENUS[0].clone();
    let mut option_index = 0i32;

    log::info!("Configuring terminal...");
    configure_terminal(&mut stdout)?;

    log::info!("Entering main loop of TUI");
    loop {
        display_menu(&mut stdout, &curr_menu, &option_index)?;
        let quit_attempt = process_events(&mut curr_menu, &mut option_index)?;
        if quit_attempt {
            break
        }
    }

    log::info!("Quit attempt received. Terminating TUI...");
    Ok(())
}

fn configure_terminal(stdout: &mut Stdout) -> io::Result<()> {
    let title = "ConeRobo TUI";
    log::info!("Setting terminal title to {}", title);
    crossterm::execute!(stdout, SetTitle(title))?;
    Ok(())
}

fn display_menu(
    stdout: &mut Stdout, curr_menu: &Menu, option_index: &i32
) -> io::Result<()> {
    log::info!("Displaying menu");

    // Top border
    let top_border = create_top_border(curr_menu.title).unwrap();
    crossterm::queue!(
        stdout,
        Clear(ClearType::All),
        terminal::SetSize(10, 10),  // test
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
    let message_border = "-".repeat(get_terminal_width()? as usize);
    let bottom_border = "=".repeat(get_terminal_width()? as usize);
    crossterm::queue!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
        Print(message_border + "\n"),
        Print("\n"),
        Print(bottom_border),
        ResetColor
    )?;

    stdout.flush()?;
    Ok(())
}

fn create_top_border(title: &str) -> io::Result<String> {
    let terminal_width = get_terminal_width()? as i32;
    let padding = 2;
    let title_width = title.len() as i32 + padding;
    let mut top_border = "=".repeat(max((terminal_width - title_width) as usize, 0));
    top_border.push_str(format!(" {} ", title).as_str());

    Ok(top_border)
}

// Returns true if a quit attempt was processed.
fn process_events(curr_menu: &mut Menu, option_index: &mut i32) -> io::Result<bool> {
    // Will block until an event is received.
    log::info!("Waiting for event...");
    match read()? {
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
                                        // Action::Unavailable => println!("Option unavailable"),
                                        Action::QuitAttempt => return Ok(true),
                                        Action::Navigation { next_menu } => {
                                            match get_next_menu(next_menu) {
                                                Some(menu) => *curr_menu = menu,
                                                None => {}
                                            };
                                            *option_index = 0;
                                        }
                                    }
                                }
                                None => {
                                    log::error!(
                                        "I004: Menu option index \"{:?}\" out of bounds.",
                                        option_index
                                    )
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

fn get_next_menu(next_menu: MenuState) -> Option<Menu> {
    for menu in MENUS {
        if menu.state == next_menu {
            return Some(menu.clone())
        }
    };

    log::error!("I005: Next menu \"{}\" does not exist.", next_menu.0);
    None
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