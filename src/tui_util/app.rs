use super::super::tui_util;
use super::super::parts;
use super::super::lua2rust;

use std::sync::mpsc;
use crossterm::event::KeyCode;
use tui::{
    backend::Backend,
    Terminal,
};
use tui_util::{
    backend,
    ui::ui,
};
use parts::*;


pub enum InputMode {
    Normal,
    Command,
}

// App holds the state of the application
// input_mode: Current input mode
// input: Current value of the input box
// output: History of recorded messages
// command: Value entered from input box
// tabs: Names of tabs
pub struct App {
    pub input_mode: InputMode,
    pub input: String,
    pub output: Vec<String>,
    pub command: String,
    pub tab_names: Vec<&'static str>,
}

impl Default for App {
    fn default() -> App {
        App {
            input_mode: InputMode::Normal,
            input: String::new(),
            output: Vec::new(),
            command: String::new(),
            tab_names: vec!["core", "parts"],
        }
    }
}

pub fn run_app<B: Backend> (terminal: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn std::error::Error>> {
    // mpsc channel to communicate between input handler and rendering loop
    let (sender, receiver) = mpsc::channel();
    let sender_clone = sender.clone();
    backend::handle_input(sender_clone);

    // Initialize parts manager
    let mut parts = PartsManager::new();

    // main loop
    loop {
        // render
        terminal.draw(|rect| ui(rect, &mut app))?;
        
        // receive input
        match receiver.recv()? {
            backend::Event::Input(event) => match app.input_mode {
                InputMode::Normal => match event.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Command;
                    },
                    KeyCode::Char('q') => {
                        backend::disable_raw()?;
                        terminal.show_cursor()?;
                        return Ok(());
                    },
                    _ => {},
                },
                InputMode::Command => match event.code {
                    KeyCode::Enter => {
                        app.command = app.input.drain(..).collect();
                    },
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    },
                    KeyCode::Backspace => {
                        app.input.pop();
                    },
                    _ => {},
                },
            },
            backend::Event::Tick => {},
        }

        // process commands
        if !app.command.is_empty() {
            // parse command
            let temp_clone = app.command.clone();
            let mut parsed_command: Vec<&str> = temp_clone.split_whitespace().collect();
            let operation = parsed_command[0];
            let operands: Vec<&str> = parsed_command.drain(1..).collect();

            // ========== Commands List ==========
            // help:         display list of commands
            // launch:       run the core and all selected parts
            // mount [part]: add part to be managed by core
            // quit:         change InputMode back to Normal
            // parts:        show discovered parts
            match operation {
                "help" => {
                    app.output.push("========== Commands List ==========".to_owned());
                    app.output.push("help:         display list of commands".to_owned());
                    app.output.push("launch:       run the core and all selected parts".to_owned());
                    app.output.push("mount [part]: add part to be managed by core".to_owned());
                    app.output.push("quit:         change InputMode back to Normal".to_owned());
                    app.output.push("parts:        show discovered parts".to_owned());
                },

                "launch" => lua2rust::launch(&parts.get_mounted_lua_parts()),

                "mount" => parts.mount(operands[0]),

                "quit" => {
                    app.input_mode = InputMode::Normal;
                },

                "parts" => {
                    app.output.push(
                        format!(
                            "Found {} parts from {} directories:", 
                            parts.get_part_names().len(), 
                            parts.get_search_paths().len()
                        )
                    );
                    for part in parts.get_part_names() {
                        app.output.push(
                            format!("{}", part)
                        );
                    }
                },

                _ => app.output.push(
                    format!("Unknown command \"{}\": type \"help\" for commands", operation)
                ),
            }

            // clear command to avoid executing several times
            app.command = String::new();
        }
    }
}
