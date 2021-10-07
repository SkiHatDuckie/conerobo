use std::io::{
    stdin, stdout, Write,
};
// use std::collections::HashMap;
use std::thread;
use std::sync::mpsc::{
    self, Sender, Receiver,
};


// check for user input in seperate thread
fn check_input(sender: Sender<String>) {
    thread::spawn(move || {
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_)    => sender.send(buf).unwrap(),
            Err(err) => println!("Failed to parse input: {}", err)
        }
    });
}


fn main() {
    // let mut memory = HashMap::new();
    let (sender, receiver): (Sender<String>, Receiver<String>) = mpsc::channel();
    let mut sender_clone = sender.clone();

    // startup text
    println!("=== ConeRobo ===");
    print!("> ");

    // begin input thread
    let mut input = String::new();
    check_input(sender_clone);

    loop {
        stdout().flush().expect("Flush failed");

        let received = receiver.try_recv();

        if !received.is_err() { input = received.unwrap(); }

        // process user input
        if !input.is_empty() {
            match input.trim() {
                "quit" => break,
                _ => println!("Unknown command \"{}\"", input.trim())
            }

            print!("> ");

            // start a new thread to continue looking for input
            sender_clone = sender.clone();
            check_input(sender_clone);
        }

        // empty input to avoid commands being executed several times
        input = String::new()
    }
}
