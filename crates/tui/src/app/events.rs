use super::{App, AppState, ModalType, VectorListItem};
use core::Payload;
use crossterm::event::{Event, KeyCode, KeyEvent};
use std::io;
use std::path::PathBuf;

// Set how many vectors to fetch per function call in list_vectors
const VECTOR_LIST_LIMIT: usize = 50;

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
                    | Some(ModalType::Error)
                    | Some(ModalType::VectorDetails) => {
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
            Some(ModalType::ConfirmDeleteDatabase) => {
                if let Err(err) = execute_selected_db_operation(app) {
                    app.modal.show_error(err.to_string());
                }
            }
            Some(ModalType::ListVectors) => {
                open_selected_vector_from_list(app)?;
            }
            Some(ModalType::Error) => app.modal.close(),
            Some(ModalType::VectorDetails) => {
                close_vector_detail_modal(app);
            }
            _ => app.modal.enable_input_mode(),
        },
        KeyCode::Esc => match app.modal.modal_type() {
            Some(ModalType::Success)
            | Some(ModalType::Failure)
            | Some(ModalType::VectorDetails)
                if app.vector_list_post_restore && !app.vector_list_items.is_empty() =>
            {
                reopen_vector_list_modal(app);
            }
            Some(ModalType::ConfirmDeleteDatabase) => {
                app.vector_list_post_restore = false;
                restore_delete_database_modal(app);
            }
            Some(ModalType::ListVectors) => {
                app.vector_list_post_restore = false;
                app.vector_list_next_offset = None;
                app.modal.close();
            }
            Some(ModalType::VectorDetails) => {
                close_vector_detail_modal(app);
            }
            _ => {
                app.vector_list_post_restore = false;
                app.modal.close();
            }
        },
        KeyCode::Up => match app.modal.modal_type() {
            Some(ModalType::DatabaseList) | Some(ModalType::DeleteDatabase) => {
                app.select_previous();
            }
            Some(ModalType::ListVectors) => {
                if app.modal.selected_index() > 0 {
                    app.modal.select_previous();
                    app.vector_list_selected_index = app.modal.selected_index();
                }
            }
            Some(ModalType::ConfirmDeleteDatabase) => {
                app.modal.set_selected_index(0, 2);
            }
            _ => {}
        },
        KeyCode::Down => match app.modal.modal_type() {
            Some(ModalType::DatabaseList) | Some(ModalType::DeleteDatabase) => {
                app.select_next();
            }
            Some(ModalType::ListVectors) => {
                let current_len = app.vector_list_items.len();
                if current_len == 0 {
                    return Ok(());
                }

                let mut max_items = current_len;
                if app.modal.selected_index() + 1 >= current_len && fetch_next_vector_page(app)? {
                    max_items = app.vector_list_items.len();
                }

                app.modal.select_next(max_items);
                app.vector_list_selected_index = app.modal.selected_index();
            }
            Some(ModalType::ConfirmDeleteDatabase) => {
                app.modal.set_selected_index(1, 2);
            }
            _ => {}
        },
        KeyCode::Left => {
            if matches!(
                app.modal.modal_type(),
                Some(ModalType::ConfirmDeleteDatabase)
            ) {
                app.modal.set_selected_index(0, 2);
            }
        }
        KeyCode::Right => {
            if matches!(
                app.modal.modal_type(),
                Some(ModalType::ConfirmDeleteDatabase)
            ) {
                app.modal.set_selected_index(1, 2);
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
            // TODO: Add semantic search
            if matches!(app.state, AppState::VectorOperations) {
                match app.vector_selected {
                    0 => {
                        if let Err(err) = initialize_vector_listing(app) {
                            app.modal.show_error(err.to_string());
                        }
                    }
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
                app.database.create_new_database(name.clone(), path)?;
                app.modal.show_success(format!("Created database '{name}'"));
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
            if let Some((name, path)) = app.database.available_databases.get(selected_index) {
                app.pending_delete_database = Some((name.clone(), path.clone()));
                app.pending_delete_database_index = Some(selected_index);
                app.modal.show_confirm_delete_database(name.clone());
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
            show_vector_info(app, id)?;
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

fn restore_delete_database_modal(app: &mut App) {
    let index = app
        .pending_delete_database_index
        .unwrap_or_else(|| app.modal.selected_index());
    app.modal.show_delete_database();
    let max_items = app.database.available_databases.len();
    app.modal
        .set_selected_index(index.min(max_items.saturating_sub(1)), max_items);
    app.pending_delete_database = None;
    app.pending_delete_database_index = None;
    app.vector_list_post_restore = false;
}

fn to_io<E: std::fmt::Debug>(e: E) -> io::Error {
    io::Error::other(format!("{e:?}"))
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
            if let Some((name, path)) = app.database.available_databases.get(selected_index) {
                app.pending_delete_database = Some((name.clone(), path.clone()));
                app.pending_delete_database_index = Some(selected_index);
                app.modal.show_confirm_delete_database(name.clone());
            }
        }
        Some(ModalType::ConfirmDeleteDatabase) => {
            let confirm = app.modal.selected_index() == 0;
            if confirm {
                if let Some((name, path)) = app.pending_delete_database.take() {
                    app.database.delete_database(&path)?;
                    app.pending_delete_database_index = None;
                    app.modal.show_success(format!("Deleted database '{name}'"));
                }
            } else {
                restore_delete_database_modal(app);
            }
        }
        _ => {}
    }
    Ok(())
}

fn initialize_vector_listing(app: &mut App) -> io::Result<()> {
    if app.database.storage_engine.is_none() {
        app.modal.show_error("No database selected!");
        return Ok(());
    }

    app.vector_list_items.clear();
    app.vector_list_next_offset = Some(0);
    app.vector_list_post_restore = false;
    app.vector_list_selected_index = 0;
    app.vector_detail = None;

    if fetch_next_vector_page(app)? {
        app.modal.show_vector_list();
        let len = app.vector_list_items.len();
        app.modal.set_selected_index(0, len);
    } else {
        app.modal
            .show_failure("No vectors found for the current query.");
    }

    Ok(())
}

fn fetch_next_vector_page(app: &mut App) -> io::Result<bool> {
    let Some(storage) = &app.database.storage_engine else {
        app.modal.show_error("No database selected!");
        return Ok(false);
    };

    let Some(offset) = app.vector_list_next_offset else {
        return Ok(false);
    };

    let response = storage
        .list_vectors(offset, VECTOR_LIST_LIMIT)
        .map_err(to_io)?;

    let Some((vectors, next_offset)) = response else {
        app.vector_list_next_offset = None;
        return Ok(false);
    };

    if vectors.is_empty() {
        app.vector_list_next_offset = None;
        return Ok(false);
    }

    let vector_count = vectors.len();

    for (id, vector) in vectors.into_iter() {
        let payload = storage.get_payload(id).map_err(to_io)?;
        app.vector_list_items.push(VectorListItem {
            id,
            vector,
            payload,
        });
    }

    if vector_count == VECTOR_LIST_LIMIT {
        app.vector_list_next_offset = Some(next_offset);
    } else {
        app.vector_list_next_offset = None;
    }

    Ok(true)
}

fn open_selected_vector_from_list(app: &mut App) -> io::Result<()> {
    let selected_index = app.modal.selected_index();
    if let Some(item) = app.vector_list_items.get(selected_index) {
        show_vector_info(app, item.id)?;
        app.vector_list_selected_index = selected_index;
        app.vector_list_post_restore =
            matches!(app.modal.modal_type(), Some(ModalType::VectorDetails));
    }
    Ok(())
}

fn reopen_vector_list_modal(app: &mut App) {
    let len = app.vector_list_items.len();
    if len == 0 {
        app.vector_list_post_restore = false;
        app.vector_detail = None;
        app.modal.close();
        return;
    }

    let desired_index = app.vector_list_selected_index.min(len.saturating_sub(1));

    app.modal.show_vector_list();
    app.modal.set_selected_index(desired_index, len);
    app.vector_list_selected_index = desired_index;
    app.vector_list_post_restore = false;
    app.vector_detail = None;
}

fn close_vector_detail_modal(app: &mut App) {
    if app.vector_list_post_restore && !app.vector_list_items.is_empty() {
        reopen_vector_list_modal(app);
    } else {
        app.vector_detail = None;
        app.vector_list_post_restore = false;
        app.modal.close();
    }
}

fn show_vector_info(app: &mut App, id: u64) -> io::Result<()> {
    let Some(storage) = &app.database.storage_engine else {
        app.modal.show_error("No database selected!");
        return Ok(());
    };

    app.vector_list_post_restore = false;

    let vector_opt = storage.get_vector(id).map_err(to_io)?;
    let payload_opt = storage.get_payload(id).map_err(to_io)?;

    if vector_opt.is_none() && payload_opt.is_none() {
        app.vector_detail = None;
        app.modal
            .show_failure(format!("Vector with id={id} not found!"));
        return Ok(());
    }

    let vector = vector_opt.unwrap_or_default();
    let payload = payload_opt;

    app.vector_detail = Some(VectorListItem {
        id,
        vector,
        payload,
    });
    app.modal.show_vector_details();
    Ok(())
}
