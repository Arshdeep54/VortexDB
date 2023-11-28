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

// pub mod db;
use super::types::Data;
use super::types::DataType;
use rocksdb::Error as err;

use bytevec::{ByteEncodable, ByteDecodable};

pub struct Database{
    pub db: DBWithThreadMode<SingleThreaded>,
    pub db_type: DataType,
    pub path: String,
}

impl Database {
    //change to create/switch database
    pub fn create_switch_database(datatype: DataType) -> Result<Database, err> {
        let mut options = Options::default();

        //Optimize RocksDB  
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512*1024*1024);

        //Create the database if not already present
        options.create_if_missing(true);

        dotenv().ok();
        let addr = env::var("DATABASE_PATH").unwrap();

        println!("\n\nCreating database...\n\n");
        //Open the database
        let database = Database{ 
            db: DB::open(&options, &addr).unwrap(),
            db_type: datatype,
            path: addr,
        };
        println!("Created database\n\n");
        return Ok(database);
    }

    pub fn view_current_path(&self){
        println!("\n\nYour current path is: {}\n\n", self.path);
    }

    pub fn insert_collection(&self, data: Data){
        if data.data_type != self.db_type{
            println!("\n\nInvalid data type for inserting into the collection\n\n");
        }
        else{
            let value = serialize(data.vector);
            match self.db.put(data.payload.as_bytes(), value.as_ref() as &[u8]){
                Ok(_) => println!("\n\nSuccessfully inserted into the collection...\n\n"),
                Err(e) => println!("\n\nAn error occurred while instering into the collection {:?}\n\n", e),
            }
        }
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

fn serialize(vector: Vec<u32>) -> Vec<u8> {

    let bytes = vector.encode::<u32>().unwrap();
    return bytes;
}

//to deserialize the vector used in get method
fn deserialize(bytes: &[u8]) -> Vec<u32> {
    let vec = <Vec<u32>>::decode::<u32>(&bytes).unwrap();
    return vec;
}