use std::env;

mod gui;
mod tui;

fn help() {
    println!("Usage:
    cargo run conerobo -- [cmd]

Commands:
    --debug : Run ConeRobo in debug mode
    --help  : Show this menu");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // No arguments passed
        1 => run_conerobo(),
        // One argument passed
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "--debug" => debug_conerobo(),
                "--help" => help(),
                _ => {
                    eprintln!("Error: Invalid command");
                    help();
                }
            }
        },
        // All other cases
        _ => help()
    }
}

fn run_conerobo() {
    gui::launch_gui();
}

fn debug_conerobo() {
    match tui::launch_debug_interface() {
        Ok(()) => {},
        Err(err) => println!("An error occurred when trying to run the debug interface: {}", err)
    }
}