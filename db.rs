//For rocks-db
use rocksdb::{
    DB, 
    Options, 
    // WriteBatch, 
    // DBPinnableSlice, 
    SingleThreaded, 
    DBWithThreadMode, 
    IteratorMode,
    Error as err,
};
use super::types::Data;
use sha2::{Sha256, Digest};

//For env file
use dotenv::dotenv;
use std::{
    env,
    path::Path,
};


pub struct Database{
    pub db: DBWithThreadMode<SingleThreaded>,
    pub path: String,
}

impl Database {
    //error correction in all cases
    //close current database on switching to a new database

    pub fn create_switch_database() -> Result<Database, err> {
        let mut options = Options::default();

        //Optimize RocksDB  
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512*1024*1024);

        //Create the database if not already present
        options.create_if_missing(true);

        dotenv().ok();
        let addr = env::var("DATABASE_PATH").unwrap();
        //Open the database
        let database = Database{ 
            db: DB::open(&options, &addr).unwrap(),
            path: addr,
        };
        return Ok(database);
    }

    pub fn view_current_path(&self){
        println!("\n\nYour current path is: {}\n\n", self.path);
    }

    pub fn insert_in_database(&self, data: Data){
        let value = serialize(data);
        let mut hasher = Sha256::new();
        hasher.update(&value);
        let key = hasher.finalize();

        match self.db.put(key, value.as_ref() as &[u8]){
            Ok(_) => println!("\nSuccessfully inserted into the database...\n"),
            Err(e) => println!("\nAn error occurred while instering into the database {:?}\n", e),
        };
        println!("Inserted data with key {:x}", key);
    }

    pub fn view_database(database: &Database){

        let iter = database.db.iterator(IteratorMode::Start); //iterates from the start
        println!("\n\nIterating over database...");
        for item in iter {
            let (_, value) = item.unwrap();
            let vec = deserialize(&value);
            println!("Saw {:?}", vec);
        }
        println!("\n");
    }

    pub fn delete_database(&self){
        //destroy the database
        let options = Options::default();
        match DB::destroy(&options, &self.path) {
            Ok(()) => println!("Database successfully deleted"),
            Err(e) => println!("An error occurred while deleting the database: {}", e),
        }
    }

}

pub fn check_path() -> bool{
    dotenv().ok();
    let file_path = env::var("DATABASE_PATH").unwrap();
    let path = Path::new(&file_path);
    return path.exists();
}

pub fn check_database() -> bool{
    dotenv().ok();
    let file_path = env::var("DATABASE_PATH").unwrap();
    let options = Options::default();

    match DB::open_for_read_only(&options, file_path, false){
        Ok(_) => return true,
        Err(_) => {
            return false;
        }
    }
}

fn serialize(vector: Data) -> Vec<u8> {
    let bytes = bincode::serialize(&vector).unwrap();
    return bytes;
}

fn deserialize(bytes: &[u8]) -> Data {
    let vec = bincode::deserialize(&bytes).unwrap();
    return vec;
}