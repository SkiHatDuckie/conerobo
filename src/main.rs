use flexi_logger::{FileSpec, Logger, WriteMode};
use log;

use std::env;

mod tui;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let _logger = Logger::try_with_str("debug")?
        .log_to_file(FileSpec::default()        // All logs will be written to a file.
            .directory("logs")
            .basename("ConeRobo"))
        .write_mode(WriteMode::BufferAndFlush)  // Reduces I/O overhead casued by logging.
        .start()?;

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
        _ => help()
    }

    Ok(())
}

fn help() {
    println!("Usage:
    cargo run conerobo -- [cmd]

Commands:
    --help  : Display this message");
}

fn run_conerobo() {
    log::info!("Running ConeRobo TUI");
    match tui::launch_user_interface() {
        Ok(()) => {},
        Err(err) => {
            log::error!("I000: Fatal error while running TUI: {:?}", err);
            println!("I000: Fatal error while running TUI. See log for details.")
        }
    }
    log::info!("Terminated ConeRobo TUI");
}