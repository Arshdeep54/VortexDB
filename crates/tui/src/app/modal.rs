use super::state::ModalType;
use ratatui::style::Color;

pub struct ModalManager {
    show_modal: bool,
    modal_type: Option<ModalType>,
    input_buffer: String,
    secondary_input: String,
    tertiary_input: String,
    active_field: usize,
    input_mode: bool,
    selected_index: usize,
    error_message: Option<String>,
}

impl ModalManager {
    pub fn new() -> Self {
        Self {
            show_modal: false,
            modal_type: None,
            input_buffer: String::new(),
            secondary_input: String::new(),
            tertiary_input: String::new(),
            active_field: 0,
            input_mode: false,
            selected_index: 0,
            error_message: None,
        }
    }

    pub fn is_showing(&self) -> bool {
        self.show_modal
    }

    pub fn modal_type(&self) -> Option<&ModalType> {
        self.modal_type.as_ref()
    }

    pub fn input_buffer(&self) -> &str {
        &self.input_buffer
    }

    pub fn secondary_input(&self) -> &str {
        &self.secondary_input
    }

    pub fn tertiary_input(&self) -> &str {
        &self.tertiary_input
    }

    pub fn active_field(&self) -> usize {
        self.active_field
    }

    pub fn input_mode(&self) -> bool {
        self.input_mode
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn show_create_database(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::CreateDatabase);
        self.input_buffer.clear();
        self.input_mode = true;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn show_get_vector(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::GetVector);
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.active_field = 0;
        self.input_mode = true;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn show_insert_vector(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::InsertVector);
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.input_mode = true;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn show_delete_vector(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::DeleteVector);
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.active_field = 0;
        self.input_mode = true;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn show_vector_list(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::ListVectors);
        self.input_mode = false;
        self.error_message = None;
        self.selected_index = 0;
    }
    pub fn show_vector_details(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::VectorDetails);
        self.input_mode = false;
        self.selected_index = 0;
        self.error_message = None;
    }
    pub fn show_success<S: Into<String>>(&mut self, message: S) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::Success);
        self.input_mode = false;
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.error_message = Some(message.into());
    }

    pub fn show_failure<S: Into<String>>(&mut self, message: S) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::Failure);
        self.input_mode = false;
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.error_message = Some(message.into());
    }

    pub fn show_database_list(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::DatabaseList);
        self.input_mode = false;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn show_delete_database(&mut self) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::DeleteDatabase);
        self.input_mode = false;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn show_confirm_delete_database(&mut self, name: String) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::ConfirmDeleteDatabase);
        self.input_mode = false;
        self.selected_index = 0;
        self.error_message = Some(format!("Delete database '{name}'?"));
    }

    pub fn show_error<S: Into<String>>(&mut self, message: S) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::Error);
        self.input_mode = false;
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.active_field = 0;
        self.selected_index = 0;
        self.error_message = Some(message.into());
    }

    pub fn close(&mut self) {
        self.show_modal = false;
        self.modal_type = None;
        self.input_buffer.clear();
        self.secondary_input.clear();
        self.tertiary_input.clear();
        self.active_field = 0;
        self.input_mode = false;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn enable_input_mode(&mut self) {
        self.input_mode = true;
    }

    pub fn add_char(&mut self, c: char) {
        match self.active_field {
            0 => self.input_buffer.push(c),
            1 => self.secondary_input.push(c),
            _ => self.tertiary_input.push(c),
        }
    }

    pub fn remove_char(&mut self) {
        match self.active_field {
            0 => {
                self.input_buffer.pop();
            }
            1 => {
                self.secondary_input.pop();
            }
            _ => {
                self.tertiary_input.pop();
            }
        }
    }

    pub fn switch_field(&mut self) {
        self.active_field = (self.active_field + 1) % 3;
    }

    pub fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn select_next(&mut self, max_items: usize) {
        if max_items > 0 {
            self.selected_index = (self.selected_index + 1).min(max_items - 1);
        }
    }

    pub fn set_selected_index(&mut self, index: usize, max_items: usize) {
        if max_items == 0 {
            self.selected_index = 0;
        } else {
            self.selected_index = index.min(max_items - 1);
        }
    }

    pub fn get_input_value(&self) -> String {
        self.input_buffer.clone()
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }

    pub fn footer_items(&self) -> Vec<(String, Color)> {
        use ModalType::*;

        match self.modal_type {
            Some(CreateDatabase) => vec![
                ("Enter Create".into(), Color::Green),
                ("Esc Cancel".into(), Color::Red),
            ],
            Some(DatabaseList) => vec![
                ("↑ Navigate".into(), Color::Gray),
                ("↓ Navigate".into(), Color::Gray),
                ("Enter Select".into(), Color::Green),
                ("Esc Cancel".into(), Color::Red),
            ],
            Some(DeleteDatabase) => vec![
                ("↑ Navigate".into(), Color::Gray),
                ("↓ Navigate".into(), Color::Gray),
                ("Enter Continue".into(), Color::Green),
                ("Esc Cancel".into(), Color::Red),
            ],
            Some(ConfirmDeleteDatabase) => vec![
                ("←/→ Toggle".into(), Color::Gray),
                ("Enter Confirm".into(), Color::Green),
                ("Esc Back".into(), Color::Red),
            ],
            Some(GetVector) => vec![
                ("Enter Fetch".into(), Color::Green),
                ("Esc Close".into(), Color::Red),
            ],
            Some(InsertVector) => vec![
                ("Tab Next".into(), Color::Gray),
                ("Enter Insert".into(), Color::Green),
                ("Esc Cancel".into(), Color::Red),
            ],
            Some(DeleteVector) => vec![
                ("Enter Delete".into(), Color::Green),
                ("Esc Cancel".into(), Color::Red),
            ],
            Some(ListVectors) => vec![
                ("↑ Scroll".into(), Color::Gray),
                ("↓ Scroll".into(), Color::Gray),
                ("Enter Details".into(), Color::Green),
                ("Esc Close".into(), Color::Red),
            ],
            Some(VectorDetails) => vec![
                ("Enter Close".into(), Color::Green),
                ("Esc Back".into(), Color::Red),
            ],
            Some(Success) | Some(Failure) | Some(Error) => vec![("Esc Dismiss".into(), Color::Red)],
            _ => vec![("Esc Cancel".into(), Color::Red)],
        }
    }
}
