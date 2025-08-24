use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;

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
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Right | KeyCode::Enter => {
                self.next_page();
            }
            KeyCode::Left => {
                self.previous_page();
            }
            KeyCode::Up => {
                self.select_previous();
            }
            KeyCode::Down => {
                self.select_next();
            }
            _ => {}
        }
        Ok(())
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
                if self.db_selected > 0 {
                    self.db_selected -= 1;
                }
            }
            AppState::VectorOperations => {
                if self.vector_selected > 0 {
                    self.vector_selected -= 1;
                }
            }
        }
    }

    fn select_next(&mut self) {
        match self.state {
            AppState::Dashboard => {}
            AppState::Database => {
                let max_items = 3; // Number of database operations
                if self.db_selected < max_items - 1 {
                    self.db_selected += 1;
                }
            }
            AppState::VectorOperations => {
                let max_items = 5; // Number of vector operations
                if self.vector_selected < max_items - 1 {
                    self.vector_selected += 1;
                }
            }
        }
    }
}
