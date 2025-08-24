use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::ListItem,
    Frame,
};

use super::components::{common_instructions, create_instructions, OperationsList, PageTitle};
use crate::app::App;

const VECTOR_OPERATIONS: &[&str] = &[
    "List All Vectors",
    "Get Vector",
    "Insert Vector",
    "Delete Vector",
    "Search Similar Vectors",
];

fn get_vector_items() -> Vec<ListItem<'static>> {
    VECTOR_OPERATIONS
        .iter()
        .map(|&op| ListItem::new(op))
        .collect()
}

pub fn render_vector_operations(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = PageTitle {
        text: "Vector Operations",
        color: Color::Magenta,
    };

    let operations_list = OperationsList {
        items: get_vector_items(),
        title: "Available Operations",
        color: Color::Magenta,
        selected: app.vector_selected,
    };

    f.render_widget(title.render(), chunks[0]);
    operations_list.render(f, chunks[1]);
    f.render_widget(create_instructions(common_instructions()), chunks[2]);
}

pub fn get_vector_operations_count() -> usize {
    VECTOR_OPERATIONS.len()
}
