use crate::tui::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);
    let block = Block::default()
        .style(Style::default().bg(Color::White).fg(Color::Black));
    frame.render_widget(block, size);
    let tab_titles = app
        .tab_titles
        .iter()
        .map(|t| {
            Spans::from(Span::styled(*t, Style::default().fg(Color::Yellow)))
        })
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.tab_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Black));
    frame.render_widget(tabs, chunks[0]);
    match app.tab_index {
        0 => draw_homepage_tab(frame, chunks[1]),
        1 => draw_components_tab(frame, chunks[1]),
        2 => draw_tracker_tab(frame, chunks[1]),
        _ => unreachable!()
    };
}

fn draw_homepage_tab<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);
    let block = Block::default()
        .title("Controls")
        .borders(Borders::ALL);
    let text = vec![
        Spans::from("Left arrow, Right arrow - Tab navigation"),
        Spans::from("Q - Quit"),
    ];
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, chunks[0]);
}

fn draw_components_tab<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(0)].as_ref())
        .split(area);
    let block = Block::default()
        .title("Component Management")
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);
    let block = Block::default()
        .title("Currently loaded components")
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[1]);
}

fn draw_tracker_tab<B: Backend>(frame: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0)].as_ref())
        .split(area);
    let block = Block::default()
        .title("Tracker")
        .borders(Borders::ALL);
    frame.render_widget(block, chunks[0]);
}