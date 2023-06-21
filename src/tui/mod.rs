use crossterm::event::poll;
use crossterm::{
    self,
    event::{read, Event, KeyCode},
    execute,
    style::{Print, SetForegroundColor, Color, ResetColor},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::{
    cmp::{min, max},
    io::{Stdout, stdout},
    time::Duration
};

mod menu;
use menu::*;

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

pub fn launch_user_interface() -> crossterm::Result<()> {
    enable_raw_mode()?;
    launch_tui(&mut stdout());
    disable_raw_mode()
}

fn launch_tui(stdout: &mut Stdout) {
    let mut curr_menu = MENUS[0].clone();
    let mut option_index = 0i32;

    loop {
        match display_menu(stdout, &curr_menu, &option_index) {
            Ok(()) => {},
            Err(err) => println!("Error: {:?}\r", err)
        }
        match process_events(&mut curr_menu, &mut option_index) {
            Ok(quit_attempt) => if quit_attempt { break },
            Err(err) => println!("Error: {:?}\r", err)
        }
    }
}

fn display_menu(
    stdout: &mut Stdout, curr_menu: &Menu, option_index: &i32
) -> crossterm::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
        Print("============ ConeRobo Custom Launch ============\n"),
        Print("Options:\n"),
        ResetColor
    )?;

    for (i, menu_option) in curr_menu.options.iter().enumerate() {
        if i == *option_index as usize {
            execute!(
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
            execute!(
                stdout,
                SetForegroundColor(Color::Rgb { r: 50, g: 50, b: 50 }),
                Print(format!("\t {}\n", menu_option.option_str)),
                ResetColor
            )?;
        }
    }

    Ok(())
}

// Returns true if a quit attempt was processed.
fn process_events(curr_menu: &mut Menu, option_index: &mut i32) -> crossterm::Result<bool> {
    // Will block until an event is received.
    match read()? {
        Event::Key(event) => {
            match event.code {
                KeyCode::Up => *option_index = max(0, *option_index - 1),
                KeyCode::Down => *option_index = min(curr_menu.options.len() as i32 - 1,
                                                     *option_index + 1),
                KeyCode::Enter => {
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
                            println!("CONEROBO ERROR: Menu option index \"{}\" out of bounds.",
                                     option_index)
                        }
                    }
                },
                _ => {}
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

// Returns `None` if `next_menu` cannot be found.
fn get_next_menu(next_menu: MenuState) -> Option<Menu> {
    for menu in MENUS {
        if menu.state == next_menu {
            return Some(menu.clone())
        }
    };

    println!("CONEROBO ERROR: Next menu \"{}\" does not exist.", next_menu.0);
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

    return (first_resize, last_resize);
}