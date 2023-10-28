use rocksdb::{
    DB, 
    Options, 
    // WriteBatch, 
    // DBPinnableSlice, 
    SingleThreaded, 
    DBWithThreadMode, 
    IteratorMode
};
// pub mod db;
use super::types::Data;
use super::types::DataType;
use rocksdb::Error as err;


pub struct Database{
    pub db: DBWithThreadMode<SingleThreaded>,
    pub dbType: DataType,
}

impl Database {
    pub fn create_collection_default(data: Data) -> Result<Database, err> {

        let path = "/Users/khushalagrawal/Desktop/Labs/vector-db/db";
        let mut options = Options::default();
        //Optimize RocksDB  
        options.increase_parallelism(12);
        options.optimize_level_style_compaction(512*1024*1024);
        //Create the database if not already present
        options.create_if_missing(true);

        //Open the database
        let mut database = Database{ 
            db: DB::open(&options, path).unwrap(),
            dbType: data.data_type,
        };
        match database.db.put(b"key1", b"value"){
            Ok(_) => {
                println!("\n\nCreating a collection...");
                println!("Vector: {:?}", data.vector);
                print!("Payload: {}", data.payload);
                println!("Data type: {:?}\n\n", data.data_type);    

                return Ok(database)
            },
            Err(e) => return Err(e),
        };
    }

    pub fn insert_collection(&self, data: Data){
        //ignoring the value of the result for now
        if data.data_type != self.dbType{
            println!("\n\nInvalid data type for inserting into the collection\n\n");
        }
        match self.db.put(b"key2", b"value2"){
            Ok(_) => println!("\n\nSuccessfully inserted into the collection...\n\n"),
            Err(e) => println!("\n\nAn error occurred while instering into the collection {:?}\n\n", e),
        }
    }

    pub fn view_collections(database: &Database){

        let iter = database.db.iterator(IteratorMode::Start); //iterates from the start
        println!("\n\nIterating over collections...");
        for item in iter {
            let (key, value) = item.unwrap();
            println!("Saw {:?} {}", std::str::from_utf8(&key).unwrap(), std::str::from_utf8(&value).unwrap());
        }
        println!("\n\n");
    }

    pub fn delete_collection(&self){
        //destroy the collection
        let path = "/Users/khushalagrawal/Desktop/Labs/vector-db/db";
        let options = Options::default();
        match DB::destroy(&options, &path) {
            Ok(()) => println!("Database successfully deleted"),
            Err(e) => println!("An error occurred while deleting the database: {}", e),
        }
    }

}