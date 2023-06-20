use std::env;

mod tui;

fn help() {
    println!("Usage:
    cargo run conerobo -- [cmd]

Commands:
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
    match tui::launch_user_interface() {
        Ok(()) => {},
        Err(err) => println!("An error occurred when trying to run the debug interface: {}", err)
    }
}