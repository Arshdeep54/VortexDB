mod db;
mod types;
use std::{io, env, io::Write};
use std::fs::OpenOptions;
// use std::ops::Deref;
use types::{Data, DataType, VectorData};
use db::{Database, check_path, check_database};
mod vectoriser;

fn main(){
    println!("Welcome");
    println!("Checking for database on specified path...");

    let database: Option<Database> = valid_database();

    if database.is_none(){
        return;
    }

    let mut database: Database = database.unwrap();
    loop {
        //delete needs to be implmented
        //databases need to be implemented

        println!("Insert in Database (1)");
        println!("View Database (2)");
        println!("Delete from Database (3)");
        println!("View current path (4)");
        println!("Switch database (5)");
        println!("Exit (0)");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1"=> {
               insert_in_database(&database);
            },
            "2"=> {
                view_databases(&database);
            },
            "3"=> {
                delete_database(&database);
            },
            "4"=> {
                Database::view_current_path(&database);
            },
            "5" => {
                let exit = change_path();
                if exit {
                    break;
                }
                database = valid_database().unwrap();
            },
            
            "0"=> break,
            _ => println!("Invalid choice"),
        }
    }

    write_env();
}

fn valid_database() -> Option<Database> {
    let mut database: Option<Database> = None;
    loop{
        if check_path(){
            println!("Path is valid, validating database...");
            if check_database(){
                println!("Database exists on current path");
                database =  Some(create_switch_database());
                break;
            }
            else {
                println!("Database does not exist on current path.\n");
                println!("Creating a new database...");
                database = Some(create_switch_database());
                break;
            }
        }
        else{
            println!("Path in .env file is invalid");
            let exit = change_path();
            if exit {
                break;
            }
        }
    }

    return database;
}

fn change_path() -> bool{
    let mut br = false;
    loop{
        println!("Enter a new path for database");
        println!("Use current working directory (1)");
        println!("Enter custom path (2)");
        println!("Abort (3)");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        match choice.trim() {
            "1" => {
                let current_dir = env::current_dir().unwrap();
                let absolute_path = current_dir.canonicalize().unwrap();
                println!("Setting path to: {}", absolute_path.display());
                env::set_var("DATABASE_PATH", absolute_path);
                break;
            },
            "2" => {
                println!("Enter path");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                env::set_var("DATABASE_PATH", input.trim());
                break;
            },
            "3" => {
                br = true;
                break;
            },
            _ => {
                println!("Invalid input\n\n");
            }
        };
    }
    return br;
}

fn write_env(){
    let variables_to_write = ["DATABASE_PATH"];

    let env_vars = env::vars();
    let file_path = ".env";
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_path).unwrap();

    let mut buffered_file = io::BufWriter::new(file);

    for (key, value) in env_vars {
        if variables_to_write.contains(&key.as_str()){
            let env_string = format!("{}={}\n", key, value);
            buffered_file.write_all(env_string.as_bytes()).unwrap();
        }
    }
}

fn create_switch_database() -> Database{
    match Database::create_switch_database(){
        Ok(database) => return database,
        Err(err) => panic!("Failed to create database as {:?}", err),
    };
}

fn view_databases(database: &Database) {
    println!("Viewing databases...");
    Database::view_database(&database);
}

fn delete_database(database: &Database) {
    println!("Deleting a database...");
    database.delete_database();
}

fn insert_in_database(database: &Database) {

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

    let data = vectoriser::vectorize(Data{vector: VectorData::default(), payload: payload, data_type: datatype});

    println!("Inserting into a database...");
    database.insert_in_database(data);

}