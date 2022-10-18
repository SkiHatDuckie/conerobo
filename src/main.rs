use std::env;

mod gui;

fn help() {
    println!("Usage:
    cargo run conerobo [cmd] [args]

Commands:
    --debug : Run ConeRobo in debug mode
    --help  : Show this menu");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // No arguments passed
        1 => gui::launch_gui(),
        // One argument passed
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "--debug" => {},
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