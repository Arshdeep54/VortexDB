mod db;
mod types;

use std::io;
use types::Data;

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
    db::create_collection(Data{vector: vec![1,2,3], payload: "Hello".to_string()});
    
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

// let _ = DB::destroy(&Options::default(), path);