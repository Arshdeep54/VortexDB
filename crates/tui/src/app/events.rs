use super::{App, AppState, ModalType};
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;
use std::path::PathBuf;

pub fn handle_event(app: &mut App, event: Event) -> io::Result<()> {
    if let Event::Key(key) = event {
        handle_key_event(app, key)?;
    }
    Ok(())
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> io::Result<()> {
    // Handle modal input first
    if app.modal.is_showing() && app.modal.input_mode() {
        return handle_modal_input(app, key);
    }

    // Handle modal navigation
    if app.modal.is_showing() {
        return handle_modal_navigation(app, key);
    }

    // Regular navigation
    match app.state {
        AppState::Database => handle_database_keys(app, key)?,
        _ => handle_general_keys(app, key),
    }

    Ok(())
}

fn handle_modal_input(app: &mut App, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Enter => match execute_modal_action(app) {
            Ok(()) => app.modal.close(),
            Err(err) => app.modal.show_error(err.to_string()),
        },
        KeyCode::Esc => {
            app.modal.close();
        }
        KeyCode::Char(c) => {
            app.modal.add_char(c);
        }
        KeyCode::Backspace => {
            app.modal.remove_char();
        }
        _ => {}
    }
    Ok(())
}

fn handle_modal_navigation(app: &mut App, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Enter => match app.modal.modal_type() {
            Some(ModalType::DatabaseList) | Some(ModalType::DeleteDatabase) => {
                if let Err(err) = execute_selected_db_operation(app) {
                    app.modal.show_error(err.to_string());
                }
            }
            Some(ModalType::Error) => app.modal.close(),
            _ => app.modal.enable_input_mode(),
        },
        KeyCode::Esc => {
            app.modal.close();
        }
        KeyCode::Up => {
            if matches!(
                app.modal.modal_type(),
                Some(ModalType::DatabaseList) | Some(ModalType::DeleteDatabase)
            ) {
                app.select_previous();
            }
        }
        KeyCode::Down => {
            if matches!(
                app.modal.modal_type(),
                Some(ModalType::DatabaseList) | Some(ModalType::DeleteDatabase)
            ) {
                app.select_next();
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_database_keys(app: &mut App, key: KeyEvent) -> io::Result<()> {
    match key.code {
        KeyCode::Enter => {
            if let Err(err) = open_database_modal(app) {
                app.modal.show_error(err.to_string());
            }
        }
        KeyCode::Up => app.select_previous(),
        KeyCode::Down => app.select_next(),
        KeyCode::Left => app.previous_page(),
        KeyCode::Right => app.next_page(),
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        _ => {}
    }
    Ok(())
}

fn handle_general_keys(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),
        KeyCode::Right | KeyCode::Enter => app.next_page(),
        KeyCode::Left => app.previous_page(),
        KeyCode::Up => app.select_previous(),
        KeyCode::Down => app.select_next(),
        _ => {}
    }
}

fn open_database_modal(app: &mut App) -> io::Result<()> {
    match app.db_selected {
        0 => app.modal.show_create_database(),
        1 => {
            app.database.load_available_databases()?;
            app.modal.show_database_list();
        }
        2 => {
            app.database.load_available_databases()?;
            app.modal.show_delete_database();
        }
        _ => {}
    }
    Ok(())
}

fn execute_modal_action(app: &mut App) -> io::Result<()> {
    match app.modal.modal_type() {
        Some(ModalType::CreateDatabase) => {
            let input = app.modal.get_input_value();
            if !input.is_empty() {
                let name = input;
                let path = PathBuf::from(format!("./databases/{name}"));
                app.database.create_new_database(name, path)?;
            }
        }
        Some(ModalType::DatabaseList) => {
            let selected_index = app.modal.selected_index();
            if let Some((name, path)) = app.database.available_databases.get(selected_index) {
                app.database.select_database(name.clone(), path.clone())?;
            }
        }
        Some(ModalType::DeleteDatabase) => {
            let selected_index = app.modal.selected_index();
            if let Some((_, path)) = app.database.available_databases.get(selected_index) {
                let path = path.clone();
                app.database.delete_database(&path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn execute_selected_db_operation(app: &mut App) -> io::Result<()> {
    match app.modal.modal_type() {
        Some(ModalType::DatabaseList) => {
            let selected_index = app.modal.selected_index();
            if let Some((name, path)) = app.database.available_databases.get(selected_index) {
                app.database.select_database(name.clone(), path.clone())?;
                app.modal.close();
            }
        }
        Some(ModalType::DeleteDatabase) => {
            let selected_index = app.modal.selected_index();
            if let Some((_, path)) = app.database.available_databases.get(selected_index) {
                let path = path.clone();
                app.database.delete_database(&path)?;
                app.modal.close();
            }
        }
        _ => {}
    }
    Ok(())
}
