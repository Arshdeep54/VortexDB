mod db;
mod types;

use std::io;
use types::Data;
use types::DataType;

fn main(){
    println!("Welcome");
    loop {
        println!("Create a collection (1)");
        println!("View collections (2)");
        println!("Delete a collection (3)");
        println!("Insert in collection (4)");
        println!("Exit (0)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1"=> create_collection(),
            "2"=> view_collections(),
            "3"=> delete_collection(),
            "4"=> insert_in_collection(),
            "0"=> break,
            _ => println!("Invalid choice"),
        }
    }
}

fn create_collection() {
    // Implement the logic to create a collection here
    println!("Creating a collection...");
    print!("Enter data type (Text, Image, Audio, Blob): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // match statement to match datatype from Datatype
    let datatype = match input.trim() {
        "Text" => DataType::Text,
        "Image" => DataType::Image,
        "Audio" => DataType::Audio,
        "Blob" => DataType::Blob,
        _ => {
            panic!("Invalid data type");
        }
    };
    print!("Enter payload: ");
    let mut payload = String::new();
    io::stdin().read_line(&mut payload).expect("Failed to read line");
    db::create_collection(Data{vector: vec![1,2,3], payload: payload, data_type: datatype});
}

fn view_collections() {
    // Implement the logic to view collections here
    println!("Viewing collections...");
}

fn delete_collection() {
    // Implement the logic to delete a collection here
    println!("Deleting a collection...");
}

fn insert_in_collection() {
    // Implement the logic to insert into a collection here
    println!("Inserting into a collection...");
}