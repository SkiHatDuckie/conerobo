use std::io::{self, Write};

#[derive(Clone)]
enum MenuState {
    MainMenu,
    GUILaunch,
    ComponentLaunch,
}

pub fn launch_debug_interface() {
    let mut input = String::new();
    let mut menu_state = MenuState::MainMenu;

    loop {
        input.clear();

        match menu_state {
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

        // We need to manually flush the stdout.
        // This ensures that inputs appear on the same line as "> ".
        match io::stdout().flush() {
            Ok(()) => {},
            Err(err) => println!("ConeRobo ERROR! Failed to flush stdout: {}", err)
        }

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim_end().to_owned();
                let mut next_menu_state = menu_state.clone();
                match menu_state {
                    MenuState::MainMenu => {
                        match input.as_str() {
                            "0" => next_menu_state = MenuState::GUILaunch,
                            "1" => next_menu_state = MenuState::ComponentLaunch,
                            "2" => println!("Option unavailable"),
                            "3" => println!("Option unavailable"),
                            "Q" => break,
                            _ => println!("Unknown command: '{}'", input)
                        }
                    },
                    MenuState::GUILaunch => {
                        match input.as_str() {
                            "Q" => next_menu_state = MenuState::MainMenu,
                            _ => println!("Unknown command: '{}'", input)
                        }
                    },
                    MenuState::ComponentLaunch => {
                        match input.as_str() {
                            "Q" => next_menu_state = MenuState::MainMenu,
                            _ => println!("Unknown command: '{}'", input)
                        }
                    }
                }
                menu_state = next_menu_state;
            },
            Err(err) => println!("ConeRobo ERROR! Failed to read input: {}", err)
        }
    }
}