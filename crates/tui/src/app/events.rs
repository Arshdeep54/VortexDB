use super::{App, AppState, ModalType};
use core::Payload;
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
            Ok(()) => {
                // keep result modals open until user dismisses them; close others
                match app.modal.modal_type() {
                    Some(ModalType::Success)
                    | Some(ModalType::Failure)
                    | Some(ModalType::Error) => {
                        // leave open
                    }
                    _ => app.modal.close(),
                }
            }
            Err(err) => app.modal.show_error(err.to_string()),
        },
        KeyCode::Esc => {
            app.modal.close();
        }
        KeyCode::Tab => {
            app.switch_field();
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
        KeyCode::Right => app.next_page(),
        KeyCode::Enter => {
            // Map options with vector operations page
            // TODO: Add listing and semantic search
            if matches!(app.state, AppState::VectorOperations) {
                match app.vector_selected {
                    1 => app.modal.show_get_vector(),
                    2 => app.modal.show_insert_vector(),
                    3 => app.modal.show_delete_vector(),
                    _ => {}
                }
            } else {
                app.next_page()
            }
        }
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
        Some(ModalType::GetVector) => {
            let input = app.modal.get_input_value();
            let vector_id_field = &input.parse::<u64>().ok();
            if vector_id_field.is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "IDs should be an integer!",
                ));
            }
            let id = vector_id_field.unwrap();
            if let Some(storage) = &app.database.storage_engine {
                // Invoke get_vector and get_payload function from storage crate to get vector
                match storage.get_vector(id).map_err(to_io) {
                    Ok(vec_opt) => match storage.get_payload(id).map_err(to_io) {
                        Ok(pay_opt) => {
                            if vec_opt.is_none() && pay_opt.is_none() {
                                app.modal
                                    .show_failure(format!("Vector with id={id} not found!"));
                            } else {
                                let vec_display = if let Some(v) = vec_opt.as_ref() {
                                    let total = v.len();
                                    let take = total.min(9);
                                    let elems = v
                                        .iter()
                                        .take(take)
                                        .map(|x| format!("{x:.2}"))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    if total > take {
                                        format!("[{elems}, ...] ({total} dims)")
                                    } else {
                                        format!("[{elems}] ({total} dims)")
                                    }
                                } else {
                                    "None".to_string()
                                };

                                let payload_display = match pay_opt.as_ref() {
                                    Some(p) => format!("{p:?}"),
                                    None => "None".to_string(),
                                };

                                app.modal.show_success(format!(
                                    "ID: {id}, Payload: {payload_display}, Vector: {vec_display}"
                                ));
                            }
                        }
                        Err(e) => {
                            app.modal.show_failure(format!("Storage error: {e}"));
                        }
                    },
                    Err(e) => {
                        app.modal.show_failure(format!("Storage error: {e}"));
                    }
                }
            } else {
                app.modal.show_error("No database selected!");
            }
        }
        Some(ModalType::InsertVector) => {
            // Form fields for insertion: id, vector, payload
            let id_text = app.modal.get_input_value();
            let vec_text = app.modal.secondary_input();
            let payload_text = app.modal.tertiary_input();

            let vector_id_field = id_text.parse::<u64>().ok();
            if vector_id_field.is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "IDs should be an integer!",
                ));
            }
            let id = vector_id_field.unwrap();

            match parse_vector(vec_text) {
                Some(vec) => {
                    if let Some(storage) = &app.database.storage_engine {
                        let payload_opt = if payload_text.trim().is_empty() {
                            None
                        } else {
                            Some(Payload {})
                        };
                        match storage
                            .insert_point(id, Some(vec), payload_opt)
                            .map_err(to_io)
                        {
                            Ok(()) => app
                                .modal
                                .show_success(format!("Inserted vector with id={id}!")),
                            Err(e) => app.modal.show_failure(format!("Storage error: {e}")),
                        }
                    } else {
                        app.modal.show_error("No database selected!");
                    }
                }
                None => {
                    app.modal
                        .show_failure("Vectors should be a list of floats (e.g. [0.1,0.2,...])!");
                }
            }
        }
        Some(ModalType::DeleteVector) => {
            let input = app.modal.get_input_value();
            let vector_id_field = &input.parse::<u64>().ok();
            if vector_id_field.is_none() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "IDs should be an integer!",
                ));
            }
            let id = vector_id_field.unwrap();
            if let Some(storage) = &app.database.storage_engine {
                match storage.contains_point(id).map_err(to_io) {
                    Ok(exists) => {
                        if !exists {
                            app.modal
                                .show_failure(format!("Vector with id={id} not found"));
                        } else {
                            match storage.delete_point(id).map_err(to_io) {
                                Ok(()) => app.modal.show_success(format!("Deleted vector id={id}")),
                                Err(e) => app.modal.show_failure(format!("Storage error: {e}")),
                            }
                        }
                    }
                    Err(e) => app.modal.show_failure(format!("Storage error: {e}")),
                }
            } else {
                app.modal.show_error("No database selected!");
            }
        }
        _ => {}
    }
    Ok(())
}

fn to_io<E: std::fmt::Debug>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{e:?}"))
}

fn parse_vector(input: &str) -> Option<Vec<f32>> {
    input
        .trim()
        .strip_prefix('[')?
        .strip_suffix(']')?
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f32>())
        .collect::<Result<Vec<_>, _>>()
        .ok()
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
