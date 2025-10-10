use crate::app::{App, ModalType};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, TableState},
};

pub fn render_modal(f: &mut Frame, app: &App) {
    if !app.show_modal() {
        return;
    }

    let size = f.size();
    let (modal_width, modal_height) = match app.modal_type() {
        Some(ModalType::ListVectors) => (85, 70),
        Some(ModalType::VectorDetails) => (80, 60),
        _ => (60, 60),
    };
    let popup_area = centered_rect(modal_width, modal_height, size);

    // Clear the area
    f.render_widget(Clear, popup_area);

    match app.modal_type() {
        Some(ModalType::CreateDatabase) => render_create_database_modal(f, app, popup_area),
        Some(ModalType::DatabaseList) => render_database_list_modal(f, app, popup_area),
        Some(ModalType::DeleteDatabase) => render_delete_database_modal(f, app, popup_area),
        Some(ModalType::ConfirmDeleteDatabase) => {
            render_confirm_delete_database_modal(f, app, popup_area)
        }
        Some(ModalType::Error) => render_error_modal(f, app, popup_area),
        Some(ModalType::GetVector) => render_get_vector_modal(f, app, popup_area),
        Some(ModalType::InsertVector) => render_insert_vector_modal(f, app, popup_area),
        Some(ModalType::SearchSimilarVectors) => {
            render_search_similar_vectors_modal(f, app, popup_area)
        }
        Some(ModalType::DeleteVector) => render_delete_vector_modal(f, app, popup_area),
        Some(ModalType::TextEmbedding) => render_text_embedding_modal(f, app, popup_area),
        Some(ModalType::SentenceEmbedding) => render_sentence_embedding_modal(f, app, popup_area),
        Some(ModalType::ImageEmbedding) => render_image_embedding_modal(f, app, popup_area),
        Some(ModalType::ListVectors) => render_vector_list_modal(f, app, popup_area),
        Some(ModalType::VectorDetails) => render_vector_details_modal(f, app, popup_area),
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
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
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
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        if app.active_field() == 0 {
            f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2);
        } else if app.active_field() == 1 {
            f.set_cursor(area.x + 1 + app.secondary_input().len() as u16, area.y + 5);
        } else {
            f.set_cursor(area.x + 1 + app.tertiary_input().len() as u16, area.y + 8);
        }
    }
}

fn render_search_similar_vectors_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Search Similar Vectors")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("Top-k (int):")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Query Vector (e.g. [0.1,0.2,...]):")),
        Line::from(Span::raw(app.secondary_input().to_string())),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        if app.active_field() == 0 {
            f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2);
        } else {
            f.set_cursor(area.x + 1 + app.secondary_input().len() as u16, area.y + 5);
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
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2);
    }
}

fn render_text_embedding_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Generate Text Embedding")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("ID (int):")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Text:")),
        Line::from(Span::raw(app.secondary_input().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Payload (optional):")),
        Line::from(Span::raw(app.tertiary_input().to_string())),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        match app.active_field() {
            0 => f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2),
            1 => f.set_cursor(area.x + 1 + app.secondary_input().len() as u16, area.y + 5),
            _ => f.set_cursor(area.x + 1 + app.tertiary_input().len() as u16, area.y + 8),
        }
    }
}

fn render_sentence_embedding_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Generate Sentence Embedding")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("ID (int):")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Sentence:")),
        Line::from(Span::raw(app.secondary_input().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Payload (optional):")),
        Line::from(Span::raw(app.tertiary_input().to_string())),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        match app.active_field() {
            0 => f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2),
            1 => f.set_cursor(area.x + 1 + app.secondary_input().len() as u16, area.y + 5),
            _ => f.set_cursor(area.x + 1 + app.tertiary_input().len() as u16, area.y + 8),
        }
    }
}

fn render_image_embedding_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Generate Image Embedding")
        .borders(Borders::ALL);

    let lines = vec![
        Line::from(Span::raw("ID (int):")),
        Line::from(Span::raw(app.input_buffer().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Image Path:")),
        Line::from(Span::raw(app.secondary_input().to_string())),
        Line::from(Span::raw("")),
        Line::from(Span::raw("Payload (optional):")),
        Line::from(Span::raw(app.tertiary_input().to_string())),
    ];

    let input = Paragraph::new(lines)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(input, area);

    if app.input_mode() {
        match app.active_field() {
            0 => f.set_cursor(area.x + 1 + app.input_buffer().len() as u16, area.y + 2),
            1 => f.set_cursor(area.x + 1 + app.secondary_input().len() as u16, area.y + 5),
            _ => f.set_cursor(area.x + 1 + app.tertiary_input().len() as u16, area.y + 8),
        }
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

fn render_vector_list_modal(f: &mut Frame, app: &App, area: Rect) {
    let has_more = app.vector_list_next_offset.is_some();
    let title = if has_more {
        "Vectors (scroll for more)"
    } else {
        "Vectors"
    };

    let header_cells = ["Dims", "ID", "Vector", "Payload"].into_iter().map(|h| {
        Cell::from(h).style(
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
    });
    let header = Row::new(header_cells).height(1);

    let rows: Vec<Row> = app
        .vector_list_items
        .iter()
        .map(|item| {
            Row::new(vec![
                Cell::from(item.dims().to_string()),
                Cell::from(item.id.to_string()),
                Cell::from(item.snippet(8)),
                Cell::from(item.payload_summary()),
            ])
        })
        .collect();

    let mut state = TableState::default();
    if !app.vector_list_items.is_empty() {
        let selected = app
            .modal
            .selected_index()
            .min(app.vector_list_items.len().saturating_sub(1));
        state.select(Some(selected));
    }

    let widths = [
        Constraint::Length(8),
        Constraint::Length(14),
        Constraint::Percentage(45),
        Constraint::Percentage(33),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .column_spacing(2)
        .highlight_symbol("▶ ")
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black));

    f.render_stateful_widget(table, area, &mut state);
}

fn render_vector_details_modal(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .title("Vector Details")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    if let Some(item) = &app.vector_detail {
        let header_cells = ["Dims", "ID", "Vector", "Payload"].into_iter().map(|h| {
            Cell::from(h).style(
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1);

        let row = Row::new(vec![
            Cell::from(item.dims().to_string()),
            Cell::from(item.id.to_string()),
            Cell::from(item.snippet(16)),
            Cell::from(item.payload_summary()),
        ])
        .height(2);
        let rows = vec![row];

        let widths = [
            Constraint::Length(8),
            Constraint::Length(14),
            Constraint::Percentage(45),
            Constraint::Percentage(33),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .column_spacing(2);

        f.render_widget(table, area);
    } else {
        let placeholder = Paragraph::new("No vector data available.")
            .block(block)
            .alignment(Alignment::Center)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(placeholder, area);
    }
}

fn render_error_modal(f: &mut Frame, app: &App, area: Rect) {
    let message = app.error_message().unwrap_or("An unknown error occurred!");
    let content = Paragraph::new(vec![Line::from(message.to_string())])
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
    let content = Paragraph::new(vec![Line::from(message.to_string())])
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
    let content = Paragraph::new(vec![Line::from(message.to_string())])
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

fn render_confirm_delete_database_modal(f: &mut Frame, app: &App, area: Rect) {
    let prompt = app
        .error_message()
        .unwrap_or("Are you sure you want to delete this database?");

    let block = Block::default()
        .title("Confirm Deletion")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let options = ["Yes", "No"];
    let selected = app
        .modal
        .selected_index()
        .min(options.len().saturating_sub(1));

    let option_line = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            if i == selected {
                Span::styled(
                    format!("[ {option} ]"),
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(format!("  {option}  "), Style::default().fg(Color::Gray))
            }
        })
        .collect::<Vec<_>>();

    let content = Paragraph::new(vec![
        Line::from(prompt.to_string()),
        Line::from(""),
        Line::from(option_line),
    ])
    .block(block)
    .alignment(Alignment::Center)
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
