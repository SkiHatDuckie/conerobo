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
use lua2rust::*;


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

    // Initialize lua to rust bindings
    let mut lua_bindings = Lua2Rust::default();

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
            // load:         load scripts and other resources for launch
            // mount [part]: add part to be managed by core
            // quit:         change InputMode back to Normal
            // parts:        show discovered parts
            match operation {
                "help" => {
                    app.output.push("========== Commands List ==========".to_owned());
                    app.output.push("help:         display list of commands".to_owned());
                    app.output.push("load:         load scripts and other resources for launch".to_owned());
                    app.output.push("mount [part]: add part to be managed by core".to_owned());
                    app.output.push("quit:         change InputMode back to Normal".to_owned());
                    app.output.push("parts:        show discovered parts".to_owned());
                },

                "load" => lua_bindings.load(parts.get_mounted_lua_parts()),

                "mount" => app.output.push(parts.mount(operands[0])),

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

        // collect sent data from luas
        if lua_bindings.is_loaded() {
            let from_lua = lua_bindings.get_from_lua();

            // check for messages for core
            for i in from_lua.iter() {
                if i.label == "MSG".to_owned() {
                    app.output.push(i.data.clone());
                }
            }
        }
    }
}
