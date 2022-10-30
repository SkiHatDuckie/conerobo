use std::io::{self, Write};

pub fn launch_debug_interface() {
    let mut input = String::new();

    loop {
        input.clear();

        print!("
============ ConeRobo Custom Launch ============
Options:
    0 > Configure GUI Launch
    1 > Configure component management launch
    2 > Launch individual core module
    3 > Launch entire core
    Q > Exit TUI
------------------------------------------------
> ");

        // We need to manually flush the stdout.
        // This ensures that inputs appear on the same line as "> ".
        match io::stdout().flush() {
            Ok(()) => {},
            Err(err) => println!("ConeRobo ERROR! Failed to flush stdout: {}", err)
        }

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim_end() {
                    "Q" => break,
                    _ => {}
                }
            },
            Err(err) => println!("ConeRobo ERROR! Failed to read input: {}", err)
        }

        println!("{} was selected!", input);
    }
}