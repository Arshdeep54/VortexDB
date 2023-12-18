//For rocks-db
use super::types::Data;
use hex::decode;
use rocksdb::{
    DBWithThreadMode,
    Error as err,
    IteratorMode,
    Options,
    // WriteBatch,
    // DBPinnableSlice,
    SingleThreaded,
    DB,
};
use sha2::{Digest, Sha256};

//For env file
use dotenv::dotenv;
use std::{collections::HashMap, env, path::Path};

pub struct Database {
    pub db: DBWithThreadMode<SingleThreaded>,
    pub path: String,
}

impl Database {
    //error correction in all cases

    pub fn create_switch_database(addr: String) -> Result<Database, err> {
        let mut options = Options::default();

        //Optimize RocksDB
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512 * 1024 * 1024);

        //Create the database if not already present
        options.create_if_missing(true);

        dotenv().ok();
        //Open the database
        let database = Database {
            db: DB::open(&options, &addr).unwrap(),
            path: addr,
        };
        return Ok(database);
    }

    pub fn view_current_path(&self) {
        println!("\n\nYour current path is: {}\n\n", self.path);
    }

    pub fn insert_in_database(&self, data: Data) {
        let value = serialize(data);
        let mut hasher = Sha256::new();
        hasher.update(&value);
        let key = hasher.finalize();

        match self.db.put(key, value.as_ref() as &[u8]) {
            Ok(_) => println!("\nSuccessfully inserted into the database...\n"),
            Err(e) => println!(
                "\nAn error occurred while instering into the database {:?}\n",
                e
            ),
        };
        println!("Inserted data with key {:x}", key);
    }

    pub fn view_database(database: &Database) {
        let iter = database.db.iterator(IteratorMode::Start); //iterates from the start
        println!("\n\nIterating over database...");
        for item in iter {
            let (_, value) = item.unwrap();
            let vec = deserialize(&value);
            println!("{:?}", vec);
        }
        println!("\n");
    }

    pub fn delete_database(&self) {
        //destroy the database
        let options = Options::default();
        match DB::destroy(&options, &self.path) {
            Ok(()) => println!("Database successfully deleted"),
            Err(e) => println!("An error occurred while deleting the database: {}", e),
        }
    }

    pub fn delete_from_database_with_key(&self, input: &str) {
        match decode(input) {
            Ok(bytes) => {
                let key = bytes.into_boxed_slice();
                match self.db.get(&key) {
                    Ok(Some(_)) => match self.db.delete(key) {
                        Ok(_) => println!("Deleted successfully"),
                        Err(e) => println!("An error occurred while deleting the database: {}", e),
                    },
                    Ok(None) => println!("Key does not exist"),
                    Err(e) => println!("Error getting key: {}", e),
                }
            }
            Err(_) => {
                println!("Invalid key");
                return;
            }
        }
    }

    pub fn get(&self, input: &str) {
        match decode(input) {
            Ok(bytes) => {
                let key = bytes.into_boxed_slice();
                match self.db.get(&key) {
                    Ok(Some(value)) => {
                        let vec = deserialize(&value);
                        println!("{:?}", vec);
                    }
                    Ok(None) => println!("Key does not exist"),
                    Err(e) => println!("Error getting key: {}", e),
                }
            }
            Err(_) => {
                println!("Invalid key");
                return;
            }
        }
    }
}

pub fn check_path(file_path: &String) -> bool {
    dotenv().ok();
    let path = Path::new(file_path);
    return path.exists();
}

pub fn check_database(file_path: &String) -> bool {
    dotenv().ok();
    let options = Options::default();

    match DB::open_for_read_only(&options, file_path, false) {
        Ok(_) => return true,
        Err(_) => {
            return false;
        }
    }
}

pub fn find_databases() -> HashMap<String, String> {
    let mut collections = HashMap::new();
    dotenv().ok();
    let count: u32 = env::var("NUMBER_OF_DATABASE").unwrap().parse().unwrap();
    for cnt in 1..(count + 1) {
        let temp: String = cnt.to_string();
        let mut db_path_var = "DATABASE_PATH".to_string();
        let mut db_name_var = "DATABASE_NAME".to_string();
        db_path_var.push_str(&temp);
        db_name_var.push_str(&temp);

        collections.insert(
            env::var(db_name_var).unwrap(),
            env::var(db_path_var).unwrap(),
        );
    }
    return collections;
}

fn serialize(vector: Data) -> Vec<u8> {
    let bytes = bincode::serialize(&vector).unwrap();
    return bytes;
}

fn deserialize(bytes: &[u8]) -> Data {
    let vec = bincode::deserialize(&bytes).unwrap();
    return vec;
}
