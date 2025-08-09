use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

#[allow(unused_variables)]
pub fn render_dashboard(f: &mut Frame, app: &App) {
    let size = f.size();

    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ])
        .split(size);

    // Create beautiful ASCII-style title
    let title_lines = vec![
        Line::from(""),
        Line::from(""),
        Line::from(vec![Span::styled(
            "██╗   ██╗███████╗ ██████╗████████╗ ██████╗ ██████╗ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║   ██║██╔════╝██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║   ██║█████╗  ██║        ██║   ██║   ██║██████╔╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "╚██╗ ██╔╝██╔══╝  ██║        ██║   ██║   ██║██╔══██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            " ╚████╔╝ ███████╗╚██████╗   ██║   ╚██████╔╝██║  ██║",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "  ╚═══╝  ╚══════╝ ╚═════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "██████╗ ██████╗ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██╔══██╗██╔══██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║  ██║██████╔╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██║  ██║██╔══██╗",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "██████╔╝██████╔╝",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(vec![Span::styled(
            "╚═════╝ ╚═════╝ ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(""),
    ];

    let title = Paragraph::new(title_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Welcome")
                .title_alignment(Alignment::Center)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Center);

    // Enhanced instructions with better styling
    let instructions = Paragraph::new(vec![
        Line::from(vec![Span::styled(
            "Press 'q' or 'Esc' to quit",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(ratatui::style::Modifier::ITALIC),
        )]),
        Line::from(vec![Span::styled(
            "Navigate with arrow keys",
            Style::default().fg(Color::DarkGray),
        )]),
    ])
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(title, chunks[1]);
    f.render_widget(instructions, chunks[2]);
}
