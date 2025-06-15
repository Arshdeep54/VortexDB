use crate::database::db_thread;
use crate::database::keygen::*;
use crate::database::types::Data;
use hex::{decode, FromHexError as hexerr};
use rocksdb::backup::{BackupEngine, BackupEngineOptions, RestoreOptions};
use rocksdb::{DBWithThreadMode, Error as err, IteratorMode, Options, SingleThreaded, DB};
use sha2::{Digest, Sha256};

pub struct Database {
    pub db: DBWithThreadMode<SingleThreaded>,
    pub name: String,
    pub backup_path: String,
    pub backup_engine: BackupEngine,
    pub wal_ttl: u64,
}

const WAL_TTL: u64 = 24 * 60 * 60;

impl Database {
    pub fn create_database(name: &str, path: &str) -> Result<Database, err> {
        let mut options = Options::default();

        //Optimize RocksDB
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512 * 1024 * 1024);

        //Create the database if not already present
        options.create_if_missing(true);

        options.set_wal_dir(path.trim());
        options.set_wal_ttl_seconds(WAL_TTL);

        // Open the DB using tmpfs
        let db = rocksdb::DB::open(&options, format!("/tmp/{}", name.trim())).unwrap();

        let backup_engine_options = BackupEngineOptions::new(path.trim()).unwrap();
        let backup_env = rocksdb::Env::new().unwrap();
        let backup_engine = BackupEngine::open(&backup_engine_options, &backup_env).unwrap();

        //Open the database
        let database = Database {
            db,
            name: name.to_string(),
            backup_path: path.to_string(),
            backup_engine,
            wal_ttl: WAL_TTL,
        };

        Ok(database)
    }

    pub fn open_database(name: &str, path: &str) -> Result<Database, err> {
        let backup_engine_options = BackupEngineOptions::new(path.trim()).unwrap();
        let backup_env = rocksdb::Env::new().unwrap();
        let mut backup_engine = BackupEngine::open(&backup_engine_options, &backup_env).unwrap();
        let mut restore_options = RestoreOptions::default();

        restore_options.set_keep_log_files(true);

        backup_engine
            .restore_from_latest_backup(format!("/tmp/{}", name), path, &restore_options)
            .unwrap();

        Database::create_database(name, path)
    }

    pub fn get_current_path(&self) -> String {
        self.backup_path.clone()
    }

    pub fn insert_in_database(&mut self, data: Data) -> Result<String, err> {
        let value = serialize(data.clone());
        let mut hasher = Sha256::new();
        hasher.update(&value);
        let key = hasher.finalize();
        let key_string = format!("{:x}", key);
        match self.db.put(key, value.as_ref() as &[u8]) {
            Ok(_) => {
                if let Err(e) =
                    db_thread::add_node_pipe((key_string.clone(), data.vector.vector), 0)
                {
                    eprintln!("Failed to add node to pipe: {}", e);
                    // Optionally, you could return an error here instead of just logging it
                }
                Ok(key_string)
            }
            Err(e) => Err(e),
        }
    }

    pub fn delete_database(&self) -> Result<(), err> {
        let options = Options::default();
        DB::destroy(&options, format!("tmp/{}", self.name))
    }

    pub fn delete_from_database_with_value(&mut self, data: Data) -> Result<Option<()>, err> {
        let value = serialize(data.clone());
        let mut hasher = Sha256::new();
        hasher.update(&value);
        let key = hasher.finalize();
        let key_string = format!("{:x}", key);

        match self.db.get(key) {
            Ok(Some(_)) => {
                if let Err(e) = db_thread::delete_node_pipe(key_string) {
                    eprintln!("Failed to delete node from pipe: {}", e);
                }
                match self.db.delete(key) {
                    Ok(_) => Ok(Some(())),
                    Err(e) => Err(e),
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }

        // Find the corresponding point and remove it from the database
    }

    pub fn delete_from_database_with_key(
        &mut self,
        input: &str,
    ) -> Result<Result<Option<()>, err>, hexerr> {
        match decode(input) {
            Ok(bytes) => {
                let key = bytes.into_boxed_slice();
                match self.db.get(&key) {
                    Ok(Some(_)) => {
                        if let Err(e) = db_thread::delete_node_pipe(input.to_string()) {
                            eprintln!("Failed to delete node from pipe: {}", e);
                        }
                        match self.db.delete(key) {
                            Ok(_) => Ok(Ok(Some(()))),
                            Err(e) => Ok(Err(e)),
                        }
                    }
                    Ok(None) => Ok(Ok(None)),
                    Err(e) => Ok(Err(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_data_from_key(&self, input: &str) -> Result<Result<Option<Data>, err>, hexerr> {
        match decode(input) {
            Ok(bytes) => {
                let key = bytes.into_boxed_slice();
                match self.db.get(&key) {
                    Ok(Some(value)) => {
                        let vec = deserialize(&value);
                        Ok(Ok(Some(vec)))
                    }
                    Ok(None) => Ok(Ok(None)),
                    Err(e) => Ok(Err(e)),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_knn(
        &self,
        k_type: u8,
        k_value: usize,
        givenvec: Vec<f32>,
    ) -> Result<String, String> {
        // Convert all this data to a string and call the pipe function
        match db_thread::get_knn_pipe(k_type, k_value, givenvec) {
            Ok(_) => Ok("Finding knn...".to_owned()),
            Err(e) => {
                eprintln!("Failed to write to named pipe: {}", e);
                Err(format!("Failed to write to named pipe: {}", e))
            }
        }
    }

    pub fn sync_with_indexer(&self) {
        // iterate through the database and send the data to the indexer
        let iter = self.db.iterator(IteratorMode::Start); //iterates from the start
        println!("Syncing database with indexer...");
        for item in iter {
            let (key, value) = item.unwrap();
            let hex_strings: Vec<String> = key.iter().map(|b| format!("{:02x}", b)).collect();
            let result = hex_strings.join("");
            let vec = deserialize(&value);
            if let Err(e) = db_thread::add_node_pipe((result, vec.vector.vector), 0) {
                eprintln!("Failed to add node to pipe during sync: {}", e);
            }
        }
        println!("Syncing complete.");
    }
}
