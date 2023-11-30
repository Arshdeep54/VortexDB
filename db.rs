use rocksdb::{
    DB, 
    Options, 
    // WriteBatch, 
    // DBPinnableSlice, 
    SingleThreaded, 
    DBWithThreadMode, 
    IteratorMode
};
use dotenv::dotenv;
use std::env;
use std::path::Path;
use std::sync::atomic::AtomicU64;

use crate::types::VectorData;

// pub mod db;
use super::types::Data;
use rocksdb::Error as err;

//unique id
static ID: AtomicU64= AtomicU64::new(1);



pub struct Database{
    pub db: DBWithThreadMode<SingleThreaded>,
    pub path: String,
}

impl Database {
    //error correction in all cases
    //remove datatype
    //serialize and deserialize datatypes
    //return node key/id


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

    pub fn insert_collection(&self, data: Data) -> u64{
        let value = serialize(data);
        println!("{:?}", value);
        let key = ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        match self.db.put(key.to_string(), value.as_ref() as &[u8]){
            Ok(_) => println!("\n\nSuccessfully inserted into the collection...\n\n"),
            Err(e) => println!("\n\nAn error occurred while instering into the collection {:?}\n\n", e),
        };
        return key;
    }

    pub fn view_collections(database: &Database){

        let iter = database.db.iterator(IteratorMode::Start); //iterates from the start
        println!("\n\nIterating over collections...");
        for item in iter {
            let (key, value) = item.unwrap();
            let vec = deserialize(&value);
            println!("Saw {:?} {:?}", std::str::from_utf8(&key).unwrap(), vec);
        }
        println!("\n\n");
    }

    pub fn delete_collection(&self){
        //destroy the collection
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

    //write serialize function here
    let bytes = bincode::serialize(&vector).unwrap();
    return bytes;
}

//to deserialize the vector used in get method
fn deserialize(bytes: &[u8]) -> Data {
    let vec = bincode::deserialize(&bytes).unwrap();
    return vec;
}