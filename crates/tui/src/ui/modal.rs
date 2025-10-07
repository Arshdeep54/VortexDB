use crate::app::{App, ModalType};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
};

pub fn render_modal(f: &mut Frame, app: &App) {
    if !app.show_modal() {
        return;
    }

    let size = f.size();
    let popup_area = centered_rect(60, 60, size);

    // Clear the area
    f.render_widget(Clear, popup_area);

    match app.modal_type() {
        Some(ModalType::CreateDatabase) => render_create_database_modal(f, app, popup_area),
        Some(ModalType::DatabaseList) => render_database_list_modal(f, app, popup_area),
        Some(ModalType::DeleteDatabase) => render_delete_database_modal(f, app, popup_area),
        Some(ModalType::Error) => render_error_modal(f, app, popup_area),
        Some(ModalType::GetVector) => render_get_vector_modal(f, app, popup_area),
        Some(ModalType::InsertVector) => render_insert_vector_modal(f, app, popup_area),
        Some(ModalType::DeleteVector) => render_delete_vector_modal(f, app, popup_area),
        Some(ModalType::Success) => render_success_modal(f, app, popup_area),
        Some(ModalType::Failure) => render_failure_modal(f, app, popup_area),
        _ => {}
    }
}

fn render_create_database_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Create New Database")
        .borders(Borders::ALL);

    let input = Paragraph::new(app.input_buffer())
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(input, area);

    if app.input_mode() {
        f.set_cursor(area.x + app.input_buffer().len() as u16 + 1, area.y + 1);
    }
}

fn render_get_vector_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Get Vector by ID")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("ID:")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(""),
        Line::from(Span::styled(
            "Press Tab to switch field (if any). Press Enter to submit.",
            Style::default().fg(Color::Gray),
        )),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        // content line is the second line of the paragraph => area.y + 2
        f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2);
    }
}

fn render_insert_vector_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Insert Vector")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("ID (int):")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Vector (e.g. [0.1,0.2,...]):")),
        Line::from(Span::raw(app.secondary_input().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Payload (optional):")),
        Line::from(Span::raw(app.tertiary_input().to_string())),
        Line::from(""),
        Line::from(Span::styled(
            "Use Tab to switch fields. Enter to submit.",
            Style::default().fg(Color::Gray),
        )),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        // Here active_field: 0 -> ID, 1 -> Vector, 2 -> Payload
        // Paragraph layout: label (y+1), content (y+2), blank (y+3), next label (y+4), next content (y+5), ...
        if app.active_field() == 0 {
            // ID content is at line index 1 -> y + 2
            f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2);
        } else if app.active_field() == 1 {
            // Vector content is at line index 4 -> y + 5
            f.set_cursor(area.x + 1 + app.secondary_input().len() as u16, area.y + 5);
        } else {
            // Payload content is at line index 7 -> y + 8
            f.set_cursor(area.x + 1 + app.tertiary_input().len() as u16, area.y + 8);
        }
    }
}

fn render_delete_vector_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Delete Vector by ID")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("ID:")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter to delete",
            Style::default().fg(Color::Gray),
        )),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        // content line is the second line of the paragraph => area.y + 2
        f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2);
    }
}

fn render_database_list_modal(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .available_databases()
        .iter()
        .enumerate()
        .map(|(i, (name, _))| {
            let style = if i == app.modal.selected_index() {
                Style::default().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default()
            };
            ListItem::new(name.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Select Database")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));

    f.render_widget(list, area);
}

fn render_delete_database_modal(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .available_databases()
        .iter()
        .enumerate()
        .map(|(i, (name, _))| {
            let style = if i == app.modal.selected_index() {
                Style::default().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default()
            };
            ListItem::new(name.as_str()).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Delete Database")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));

    f.render_widget(list, area);
}

fn render_error_modal(f: &mut Frame, app: &App, area: Rect) {
    let message = app.error_message().unwrap_or("An unknown error occurred!");
    let content = Paragraph::new(vec![
        Line::from(message.to_string()),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to dismiss",
            Style::default().fg(Color::Gray),
        )),
    ])
    .block(
        Block::default()
            .title(Span::styled(
                "Error",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)),
    )
    .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(content, area);
}

fn render_success_modal(f: &mut Frame, app: &App, area: Rect) {
    let message = app.error_message().unwrap_or("Success");
    let content = Paragraph::new(vec![
        Line::from(message.to_string()),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to dismiss",
            Style::default().fg(Color::Gray),
        )),
    ])
    .block(
        Block::default()
            .title(Span::styled(
                "Success",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    )
    .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(content, area);
}

fn render_failure_modal(f: &mut Frame, app: &App, area: Rect) {
    let message = app.error_message().unwrap_or("Failure");
    let content = Paragraph::new(vec![
        Line::from(message.to_string()),
        Line::from(""),
        Line::from(Span::styled(
            "Press Esc to dismiss",
            Style::default().fg(Color::Gray),
        )),
    ])
    .block(
        Block::default()
            .title(Span::styled(
                "Failure",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red)),
    )
    .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(content, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
