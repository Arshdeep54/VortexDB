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
