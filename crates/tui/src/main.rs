mod app;
mod ui;
use app::{App, AppState};
use color_eyre::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use ui::{
    dashboard::render_dashboard, db::render_database, vector_operations::render_vector_operations,
};

const POLL_DURATION: std::time::Duration = std::time::Duration::from_millis(50);

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    let result = run_app(&mut terminal, &mut app);

    restore_terminal(&mut terminal)?;

    if let Err(err) = result {
        eprintln!("Application error: {err:?}");
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| match app.state {
            AppState::Dashboard => render_dashboard(f, app),
            AppState::Database => render_database(f, app),
            AppState::VectorOperations => render_vector_operations(f, app),
        })?;

        if event::poll(POLL_DURATION)? {
            app.handle_event(event::read()?)?;
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}
