use std::io::{
    stdin, stdout, Write,
};

fn main() {
    println!("=== ConeRobo ===");
    let mut input = String::new();

    loop {
        print!(">");
        stdout().flush().expect("Flush failed");

        // Attempt to read user input
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => println!("Failed to parse input: {}", err)
        }

        // Process user input
        match input.trim() {
            "quit" => break,
            _ => ()
        }
    }
}
