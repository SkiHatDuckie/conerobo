mod error;
mod tui;

use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, WriteMode};
use log;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let _logger = Logger::try_with_str("debug")?
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("ConeRobo")
        )
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(3)
        )
        .write_mode(WriteMode::BufferAndFlush)  // Reduces I/O overhead casued by logging.
        .start()?;

    let args = Vec::from_iter(env::args());
    let args = args.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    parse_args(&args[..]);

    Ok(())
}

fn parse_args(args: &[&str]) {
    match args.len() {
        // No arguments passed
        1 => run_conerobo(),
        // One argument passed
        2 => {
            match args[1] {
                "--help" => help(),
                _ => {
                    eprintln!("Error: Invalid command");
                    help();
                }
            }
        },
        _ => help()
    }
}

fn help() {
    println!(r#"
Usage:
    conerobo --[cmd]

Commands:
    --help  : Display this message"#);
}

fn run_conerobo() {
    log::info!("Launching ConeRobo TUI");
    tui::launch_user_interface()
        .map_err(|err| {
            log::error!("Fatal error while running TUI: {:?}", err);
            err
        })
        .unwrap();
}