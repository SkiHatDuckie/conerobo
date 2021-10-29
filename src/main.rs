// main
use std::io::{
    stdin, stdout, Write,
};
use std::thread;
// use std::collections::HashMap;
use std::sync::mpsc::{
    self, Sender, Receiver,
};

mod lua2rust;

mod parts;
use parts::*;

mod cli;
use cli::*;


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

    let mut cli = Cli::init().unwrap();

    cli.output("Searching for parts...");
    let parts = Parts::new();

    cli.output("Checking for lua scripts...");
    let luas = parts.get_lua_parts();

    println!("=== ConeRobo ===");
    print!("> ");

    // begin input thread
    let mut input = String::new();
    check_input(sender_clone);

    loop {
        stdout().flush().expect("Flush failed");

        let received = receiver.try_recv();

        if !received.is_err() {
            input = received.unwrap();
        }

        // process input
        if !input.is_empty() {
            //  === Commands List ===
            //  help:   display list of commands
            //  launch: run the core and all selected parts
            //  quit:   terminate process
            //  parts:  show discovered parts
            match input.trim() {
                "help" => println!(
                    "=== Commands List === \n\
                    help:   display list of commands \n\
                    launch: run the core and all selected parts \n\
                    quit:   terminate process \n\
                    parts:  show discovered parts"
                ),

                "launch" => lua2rust::launch(&luas),

                "quit" => break,

                "parts" => {
                    println!(
                        "Found {} parts from {} directories:", 
                        parts.get_part_names().len(), 
                        parts.get_search_paths().len()
                    );
                    for part in parts.get_part_names() {
                        println!("{}", part)
                    }
                },

                _ => println!("Unknown command \"{}\": type \"help\" for commands", input.trim())
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
