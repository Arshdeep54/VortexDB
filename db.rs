// pub mod db;
use super::types::Data;

pub fn create_collection(data: Data) {
    println!("Creating a collection...");
    println!("Vector: {:?}", data.vector);
    print!("Payload: {}", data.payload);
    println!("Data type: {:?}", data.data_type);
}