use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use storage::{create_storage_engine, StorageEngine, StorageType};

pub struct DatabaseManager {
    pub current_db_path: Option<PathBuf>,
    pub storage_engine: Option<Arc<dyn StorageEngine>>,
    pub available_databases: Vec<(String, PathBuf)>,
    pub selected_database: Option<(String, PathBuf)>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            current_db_path: None,
            storage_engine: None,
            available_databases: Vec::new(),
            selected_database: None,
        }
    }

    pub fn create_new_database(&mut self, name: String, path: PathBuf) -> io::Result<()> {
        // Check if database already exists to avoid name collisions
        if path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Database '{name}' already exists!"),
            ));
        }

        match create_storage_engine(StorageType::RocksDb, &path) {
            Ok(storage) => {
                self.storage_engine = Some(storage);
                self.current_db_path = Some(path.clone());
                self.available_databases.push((name.clone(), path.clone()));
                self.selected_database = Some((name, path));
                Ok(())
            }
            Err(e) => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to create database: {:?}", e),
            )),
        }
    }

    pub fn select_database(&mut self, name: String, path: PathBuf) -> io::Result<()> {
        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Database path does not exist",
            ));
        }

        self.selected_database = Some((name, path));
        Ok(())
    }

    pub fn delete_database(&mut self, path: &PathBuf) -> io::Result<()> {
        // Close current connection if it's the same database
        if let Some(current_path) = &self.current_db_path {
            if current_path == path {
                self.storage_engine = None;
                self.current_db_path = None;
            }
        }

        // Remove from available databases list
        self.available_databases
            .retain(|(_, db_path)| db_path != path);

        // Delete the database directory
        if path.exists() {
            std::fs::remove_dir_all(path)?;
        }

        Ok(())
    }

    pub fn load_available_databases(&mut self) -> io::Result<()> {
        let db_dir = PathBuf::from("./databases");
        if !db_dir.exists() {
            std::fs::create_dir_all(&db_dir)?;
        }

        self.available_databases.clear();

        for entry in std::fs::read_dir(&db_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    self.available_databases
                        .push((name.to_string_lossy().to_string(), path));
                }
            }
        }

        Ok(())
    }

    pub fn get_selected_database_name(&self) -> Option<&str> {
        self.selected_database
            .as_ref()
            .map(|(name, _)| name.as_str())
    }

    pub fn is_database_selected(&self) -> bool {
        self.selected_database.is_some()
    }
}
