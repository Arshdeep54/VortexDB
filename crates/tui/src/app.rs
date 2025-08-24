use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;

use crate::ui::{db, vector_operations};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AppState {
    #[default]
    Dashboard,
    Database,
    VectorOperations,
}

#[derive(Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub state: AppState,
    pub db_selected: usize,
    pub vector_selected: usize,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_event(&mut self, event: Event) -> io::Result<()> {
        if let Event::Key(key) = event {
            self.handle_key_event(key)?;
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit(),
            KeyCode::Right | KeyCode::Enter => self.next_page(),
            KeyCode::Left => self.previous_page(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            _ => {}
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.should_quit = true;
    }

    fn next_page(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Database,
            AppState::Database => AppState::VectorOperations,
            AppState::VectorOperations => AppState::VectorOperations,
        };
    }

    fn previous_page(&mut self) {
        self.state = match self.state {
            AppState::Dashboard => AppState::Dashboard,
            AppState::Database => AppState::Dashboard,
            AppState::VectorOperations => AppState::Database,
        };
    }

    fn select_previous(&mut self) {
        match self.state {
            AppState::Dashboard => {}
            AppState::Database => {
                self.db_selected = self.db_selected.saturating_sub(1);
            }
            AppState::VectorOperations => {
                self.vector_selected = self.vector_selected.saturating_sub(1);
            }
        }
    }

    fn select_next(&mut self) {
        match self.state {
            AppState::Dashboard => {}
            AppState::Database => {
                let max_items = db::get_db_operations_count();
                self.db_selected = (self.db_selected + 1).min(max_items - 1);
            }
            AppState::VectorOperations => {
                let max_items = vector_operations::get_vector_operations_count();
                self.vector_selected = (self.vector_selected + 1).min(max_items - 1);
            }
        }
    }

    pub fn get_selected_operation(&self) -> Option<usize> {
        match self.state {
            AppState::Dashboard => None,
            AppState::Database => Some(self.db_selected),
            AppState::VectorOperations => Some(self.vector_selected),
        }
    }
}
