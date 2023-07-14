use crate::tui::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs},
    Frame,
};

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
            ].as_ref()
        )
        .split(size);
    let block = Block::default()
        .style(Style::default()
            .bg(Color::White)
            .fg(Color::Black)
        );
    frame.render_widget(block, size);
    let tab_titles = app
        .tab_titles
        .iter()
        .map(|t| {
            Spans::from(
                Span::styled(*t, Style::default().fg(Color::Yellow))
            )
        })
        .collect();
    let tabs = Tabs::new(tab_titles)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Tabs")
        )
        .select(app.tab_index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Black)
        );
    frame.render_widget(tabs, chunks[0]);
    let inner = match app.tab_index {
        0 => Block::default().title("ConeRobo Homepage").borders(Borders::ALL),
        1 => Block::default().title("Components").borders(Borders::ALL),
        2 => Block::default().title("Tracker").borders(Borders::ALL),
        _ => unreachable!()
    };
    frame.render_widget(inner, chunks[1]);
}