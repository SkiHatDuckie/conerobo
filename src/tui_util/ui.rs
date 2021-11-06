use super::super::tui_util;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, ListItem, List},
    text::{Span, Spans, Text},
    Frame,
};
use tui_util::app::App;
use unicode_width::UnicodeWidthStr;


pub fn ui<B: Backend> (rect: &mut Frame<B>, app: &App) {
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
}
