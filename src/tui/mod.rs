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
use menu::{MENUS, Menu, State};

pub fn launch_debug_interface() -> crossterm::Result<()> {
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

fn display_menu(stdout: &mut Stdout, curr_menu: &Menu, option_index: &i32) -> crossterm::Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
        Print("============ ConeRobo Custom Launch ============\n"),
        Print("Options:\n"),
        ResetColor
    )?;

    for (i, s) in curr_menu.options.vec.iter().enumerate() {
        if i == *option_index as usize {
            execute!(
                stdout,
                SetForegroundColor(Color::Rgb { r: 224, g: 210, b: 58 }),
                Print("\t["),
                SetForegroundColor(Color::Rgb { r: 227, g: 227, b: 227 }),
                Print(s),
                SetForegroundColor(Color::Rgb { r: 224, g: 210, b: 58 }),
                Print("]\n"),
                ResetColor
            )?;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::Rgb { r: 50, g: 50, b: 50 }),
                Print(format!("\t {s} \n")),
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
                KeyCode::Down => *option_index = min(curr_menu.options.vec.len() as i32 - 1, *option_index + 1),
                KeyCode::Enter => {
                    match curr_menu.state {
                        State::MainMenu => {
                            match option_index {
                                0 => {
                                    *curr_menu = MENUS[1].clone();
                                    *option_index = 0;
                                },
                                1 => {
                                    *curr_menu = MENUS[2].clone();
                                    *option_index = 0;
                                },
                                2 => println!("Option unavailable"),
                                3 => println!("Option unavailable"),
                                4 => return Ok(true),
                                _ => {}
                            }
                        },
                        State::GUILaunch => {
                            match option_index {
                                0 => {
                                    *curr_menu = MENUS[0].clone();
                                    *option_index = 0;
                                },
                                _ => {}
                            }
                        },
                        State::ComponentLaunch => {
                            match option_index {
                                0 => {
                                    *curr_menu = MENUS[0].clone();
                                    *option_index = 0;
                                },
                                _ => {}
                            }
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