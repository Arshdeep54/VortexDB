use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

#[allow(unused_variables)]
pub fn render_database(f: &mut Frame, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("Database Management")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Vector DB")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Green)),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Green));

    let db_items = vec![
        ListItem::new("Create New Database"),
        ListItem::new("Open Existing Database"),
        ListItem::new("Delete Database"),
    ];

    let db_list = List::new(db_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Database Operations")
                .border_style(Style::default().fg(Color::Green)),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow));

    let instructions = Paragraph::new(vec![Line::from(vec![
        Span::styled("← Previous", Style::default().fg(Color::Gray)),
        Span::raw(" | "),
        Span::styled("→ Next", Style::default().fg(Color::Gray)),
        Span::raw(" | "),
        Span::styled("q/Esc Quit", Style::default().fg(Color::Red)),
    ])])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(title, chunks[0]);
    f.render_widget(db_list, chunks[1]);
    f.render_widget(instructions, chunks[2]);
}
