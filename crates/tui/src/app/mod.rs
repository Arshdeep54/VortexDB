mod database;
mod events;
mod modal;
mod state;

pub use state::*;

use crate::ui::{db, vector_operations};
use crossterm::event::Event;
use std::io;
use std::path::PathBuf;

pub struct App {
    pub should_quit: bool,
    pub state: AppState,
    pub db_selected: usize,
    pub vector_selected: usize,
    pub database: database::DatabaseManager,
    pub modal: modal::ModalManager,

    pub vector_list_items: Vec<VectorListItem>,
    pub vector_list_next_offset: Option<u64>,
    pub vector_list_post_restore: bool,
    pub vector_list_selected_index: usize,
    pub vector_detail: Option<VectorListItem>,
    pub pending_delete_database: Option<(String, PathBuf)>,
    pub pending_delete_database_index: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            state: AppState::default(),
            db_selected: 0,
            vector_selected: 0,
            database: database::DatabaseManager::new(),
            modal: modal::ModalManager::new(),

            vector_list_items: Vec::new(),
            vector_list_next_offset: None,
            vector_list_post_restore: false,
            vector_list_selected_index: 0,
            vector_detail: None,
            pending_delete_database: None,
            pending_delete_database_index: None,
        }
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        events::handle_event(self, event)
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_page(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => {
                let _ = self.database.load_available_databases();
                AppState::Database
            }
            AppState::Database => AppState::VectorOperations,
            AppState::VectorOperations => AppState::VectorOperations,
        };
    }

    pub fn previous_page(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Dashboard,
            AppState::Database => AppState::Dashboard,
            AppState::VectorOperations => AppState::Database,
        };
    }

    pub fn select_previous(&mut self) {
        match self.state {
            AppState::Dashboard => {}
            AppState::Database => {
                if self.modal.is_showing() {
                    self.modal.select_previous();
                } else {
                    self.db_selected = self.db_selected.saturating_sub(1);
                }
            }
            AppState::VectorOperations => {
                self.vector_selected = self.vector_selected.saturating_sub(1);
            }
        }
    }

    pub fn select_next(&mut self) {
        match self.state {
            AppState::Dashboard => {}
            AppState::Database => {
                if self.modal.is_showing() {
                    let max_items = self.database.available_databases.len();
                    self.modal.select_next(max_items);
                } else {
                    let max_items = db::get_db_operations_count();
                    self.db_selected = (self.db_selected + 1).min(max_items - 1);
                }
            }
            AppState::VectorOperations => {
                let max_items = vector_operations::get_vector_operations_count();
                self.vector_selected = (self.vector_selected + 1).min(max_items - 1);
            }
        }
    }

    pub fn show_modal(&self) -> bool {
        self.modal.is_showing()
    }

    pub fn modal_type(&self) -> Option<&ModalType> {
        self.modal.modal_type()
    }

    pub fn input_buffer(&self) -> &str {
        self.modal.input_buffer()
    }

    pub fn input_mode(&self) -> bool {
        self.modal.input_mode()
    }

    pub fn secondary_input(&self) -> &str {
        self.modal.secondary_input()
    }

    pub fn tertiary_input(&self) -> &str {
        self.modal.tertiary_input()
    }

    pub fn active_field(&self) -> usize {
        self.modal.active_field()
    }

    pub fn switch_field(&mut self) {
        self.modal.switch_field();
    }

    pub fn available_databases(&self) -> &[(String, PathBuf)] {
        &self.database.available_databases
    }

    pub fn error_message(&self) -> Option<&str> {
        self.modal.error_message()
    }

    pub fn modal_footer_items(&self) -> Vec<(String, ratatui::style::Color)> {
        self.modal.footer_items()
    }
}
