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
}
