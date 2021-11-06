use super::super::tui_util;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, ListItem, List, Tabs},
    text::{Span, Spans},
    Frame,
    symbols::line,
};
use tui_util::app::*;
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
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(size);

    // tabs
    let titles = app.tab_names.iter().cloned().map(Spans::from).collect();
    let tabs = Tabs::new(titles)
        .block(Block::default()
            .borders(Borders::ALL))
        .style(Style::default()
            .fg(Color::LightBlue))
        .highlight_style(Style::default()
            .fg(Color::Yellow))
        .divider(line::VERTICAL);
    rect.render_widget(tabs, chunks[0]);

    // input block
    let input_text = Spans::from(vec![
        Span::styled("Press ", Style::default()),
        Span::styled("Enter", Style::default()
            .add_modifier(Modifier::BOLD)),
        Span::styled(" to enter into command mode", Style::default()),
    ]);
    let input = match app.input_mode {
        InputMode::Normal => {
            Paragraph::new(input_text)
                .style(Style::default()
                    .fg(Color::LightBlue))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Input"))
        },
        InputMode::Command => {
            Paragraph::new(app.input.as_ref())
                .style(Style::default()
                    .fg(Color::LightBlue))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Input"))
        },
    };
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
            .title("Output"))
        .style(Style::default()
            .fg(Color::Blue));
    rect.render_widget(output, chunks[1]);
}
