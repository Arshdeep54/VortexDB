mod db;
mod types;
use std::io;
// use std::ops::Deref;
use types::Data;
use types::DataType;
use types::VectorData;
use db::Database;
mod vectoriser;

fn main(){
    println!("Welcome");
    let mut database: Option<Database> = None;
    loop {
        println!("Create a collection (1)");
        println!("View collections (2)");
        println!("Delete a collection (3)");
        println!("Insert in collection (4)");
        println!("View current path (5)");
        println!("Exit (0)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1"=> {
                database = Some(create_switch_collection());
            },
            "2"=> {
                match database {
                    Some(_) => view_collections(database.as_ref().unwrap()),
                    None => println!("Error in retreiving collection"),
                }
            },
            "3"=> {
                match database {
                    Some(_) => {
                        delete_collection(database.as_ref().unwrap());
                        database = None;
                    },
                    None => println!("Error in retreiving collection"),
                }
            },
            "4"=> {
                match database {
                    Some(_) => insert_in_collection(database.as_ref().unwrap()),
                    None => println!("Error in retreiving collection"),
                }
            },
            "5" => {
                match database {
                    Some(_) => Database::view_current_path(database.as_ref().unwrap()),
                    None => println!("Currently no database targeted"),
                }
            },
            
            "0"=> break,
            _ => println!("Invalid choice"),
        }
    }
}

fn create_switch_collection() -> Database{

    //need to check if the database already exists

    println!("Enter data type (Text, Image, Audio, Blob): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let datatype = match input.trim() {
        "Text" => DataType::Text,
        "Image" => DataType::Image,
        "Audio" => DataType::Audio,
        "Blob" => DataType::Blob,
        _ => {
            panic!("Invalid data type");
        }
    };

    match Database::create_switch_database(datatype){
        Ok(database) => return database,
        Err(err) => panic!("Failed to create collection as {:?}", err),
    };
}

fn view_collections(database: &Database) {
    println!("Viewing collections...");
    Database::view_collections(&database);
}

fn delete_collection(database: &Database) {
    println!("Deleting a collection...");
    database.delete_collection();
}

fn insert_in_collection(database: &Database) {

    println!("Enter data type (Text, Image, Audio, Blob): ");
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
    println!("Enter payload: ");
    let mut payload = String::new();
    io::stdin().read_line(&mut payload).expect("Failed to read line");

    let data = vectoriser::vectorize(Data{vector: vec![1,2,3], payload: payload, data_type: datatype});

    database.insert_collection(data);
    println!("Inserting into a collection...");

}