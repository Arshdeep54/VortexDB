use super::state::ModalType;

pub struct ModalManager {
    show_modal: bool,
    modal_type: Option<ModalType>,
    input_buffer: String,
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

    pub fn show_error<S: Into<String>>(&mut self, message: S) {
        self.show_modal = true;
        self.modal_type = Some(ModalType::Error);
        self.input_mode = false;
        self.input_buffer.clear();
        self.selected_index = 0;
        self.error_message = Some(message.into());
    }

    pub fn close(&mut self) {
        self.show_modal = false;
        self.modal_type = None;
        self.input_buffer.clear();
        self.input_mode = false;
        self.selected_index = 0;
        self.error_message = None;
    }

    pub fn enable_input_mode(&mut self) {
        self.input_mode = true;
    }

    pub fn add_char(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    pub fn remove_char(&mut self) {
        self.input_buffer.pop();
    }

    pub fn select_previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn select_next(&mut self, max_items: usize) {
        if max_items > 0 {
            self.selected_index = (self.selected_index + 1).min(max_items - 1);
        }
    }

    pub fn get_input_value(&self) -> String {
        self.input_buffer.clone()
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_deref()
    }
}
