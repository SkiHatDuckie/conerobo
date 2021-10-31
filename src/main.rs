// Main
mod lua2rust;
mod parts;
mod tui_util;

use std::{
    io,
    sync::mpsc,
};
use crossterm::event::KeyCode;
use tui::{
    backend::CrosstermBackend,
    Terminal,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, ListItem, List},
    text::{Span, Spans, Text},
};
use unicode_width::UnicodeWidthStr;
use parts::*;
use tui_util::backend;


// App holds the state of the application
pub struct App {
    // Current value of the input box
    pub input: String,
    // History of recorded messages
    pub output: Vec<String>,
    // Value entered from input box
    pub command: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            output: Vec::new(),
            command: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    backend::enable_raw();

    // create default app state
    let mut app = App::default();

    // mpsc channel to communicate between input handler and rendering loop
    let (sender, receiver) = mpsc::channel();
    let sender_clone = sender.clone();
    backend::handle_input(sender_clone);

    // Setup tui Terminal with CrosstermBackend
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Initialize parts manager
    let parts = Parts::new();

    // Get discovered lua script parts
    let luas = parts.get_lua_parts();

    // main loop
    loop {
        terminal.draw(|rect| {
            // set display rect size
            let size = rect.size();
            
            // divide display vertically into chunks
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);
            
            // help message
            let (msg, style) = (
                vec![
                    Span::raw("Type "),
                    Span::styled("help", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" for a list of commands. "),
                ],
                Style::default(),
            );
            let mut text = Text::from(Spans::from(msg));
            text.patch_style(style);
            let help_message = Paragraph::new(text);
            rect.render_widget(help_message, chunks[0]);

            // input block
            let input = Paragraph::new(app.input.as_ref())
                .style(Style::default()
                    .fg(Color::Cyan))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Input"));
            rect.render_widget(input, chunks[2]);

            // Make the cursor visible and ask tui-rs to put it at the specified 
            // coordinates after rendering
            rect.set_cursor(
                // Put cursor past the end of the input text
                chunks[2].x + app.input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[2].y + 1,
            );

            // output block
            let output: Vec<ListItem> = app
                .output
                .iter()
                .map(|m| {
                    let content = vec![Spans::from(Span::raw(format!("{}", m)))];
                    ListItem::new(content)
                })
                .collect();
            let output = List::new(output)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Output"));
            rect.render_widget(output, chunks[1]);
        })?;
        
        // receive input
        match receiver.recv()? {
            backend::Event::Input(event) => match event.code {
                KeyCode::Enter => {
                    app.command = app.input.drain(..).collect();
                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {},
            },
            backend::Event::Tick => {},
        }

        // process commands
        if !app.command.is_empty() {
            //  === Commands List ===
            //  help:   display list of commands
            //  launch: run the core and all selected parts
            //  quit:   terminate process
            //  parts:  show discovered parts
            match app.command.trim() {
                "help" => {
                    app.output.push("=== Commands List ===".to_owned());
                    app.output.push("help:   display list of commands".to_owned());
                    app.output.push("launch: run the core and all selected parts".to_owned());
                    app.output.push("quit:   terminate process".to_owned());
                    app.output.push("parts:  show discovered parts".to_owned());
                },

                "launch" => lua2rust::launch(&luas),

                "quit" => {
                    backend::disable_raw()?;
                    terminal.show_cursor()?;
                    break;
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
                    format!("Unknown command \"{}\": type \"help\" for commands", app.command.trim())
                ),
            }

            // clear command to avoid executing several times
            app.command.clear();
        }
    }

    Ok(())
}
