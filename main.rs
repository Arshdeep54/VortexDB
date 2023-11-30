mod db;
mod types;
use std::collections::HashMap;
use std::{io, env, io::Write};
use std::fs::OpenOptions;
// use std::ops::Deref;
use types::{Data, DataType, VectorData};
use db::{Database, check_path, check_database, find_databases};
mod vectoriser;

fn main(){
    println!("Welcome");
    println!("Finding databases...");

    let mut databases = find_databases();

    loop{
        println!("Avaiable databases are...\n");
        for (key,_) in databases.iter() {
            println!("{}",key);
        }

        println!("\nView database (1)");
        println!("Delete database (2)");
        println!("Exit (0)");

        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Failed to read line");

        match select.trim() {
            "1" => {
                println!("Enter name of database");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Faile to read line");

                if databases.contains_key(input.trim()){
                    let database: Option<Database> = valid_database(databases.get(input.trim()).unwrap());

                    if database.is_none(){
                        databases.remove(input.trim());
                        continue;
                    }

                    let database: Database = database.unwrap();
                    loop {
                        //delete needs to be implmented

                        println!("{}",input.trim());
                        println!("Insert in Database (1)");
                        println!("View Database (2)");
                        println!("Delete from Database (3)");
                        println!("View current path (4)");
                        println!("Switch database (5)");

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
                                delete_from_database(&database);
                            },
                            "4"=> {
                                Database::view_current_path(&database);
                            },
                            "5" => {
                                break;
                            },
                            _ => println!("Invalid choice"),
                        }
                    }
                    write_env(&databases);
                }
                else{
                    println!("Invalid input");
                }
            },
            "2" => {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Filed to read line");

                if databases.contains_key(input.trim()){
                    let exit = change_path();
                    if exit {
                        continue;
                    }

                    let database = valid_database(databases.get(input.trim()).unwrap()).unwrap();
                    delete_database(&database);
                }
                else{
                    println!("Invalid input");
                }
            },
            "0" => {
                break;
            },
            _ => {
                println!("Invalid choice");
            }
        };
    }
}

fn valid_database(file_path: &String) -> Option<Database> {
    let mut database: Option<Database> = None;
    loop{
        if check_path(file_path){
            println!("Path is valid, validating database...");
            if check_database(file_path){
                println!("Database exists on current path");
                database =  Some(create_switch_database(file_path.clone()));
                break;
            }
            else {
                println!("Database does not exist on current path.\n");
                println!("Creating a new database...");
                database = Some(create_switch_database(file_path.clone()));
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
        println!("Remove database (3)");
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

fn write_env(databases: &HashMap<String,String>){
    let file_path = ".env";
    let file = OpenOptions::new().write(true).truncate(true).create(true).open(file_path).unwrap();

    let mut buffered_file = io::BufWriter::new(file);

    let env_string = format!("NUMBER_OF_DATABASE={}\n", databases.len());
    buffered_file.write_all(env_string.as_bytes()).unwrap();

    let mut count = 0;

    for (key,value) in databases.iter() {
        count += 1;
        let temp: String = count.to_string();
        let mut db_path_var = "DATABASE_PATH".to_string();
        let mut db_name_var = "DATABASE_NAME".to_string();
        db_path_var.push_str(&temp);
        db_name_var.push_str(&temp);
        let env_string_path = format!("{}={}\n", db_path_var, value.clone());
        let env_string_name = format!("{}={}\n", db_name_var, key.clone());
        buffered_file.write_all(env_string_path.as_bytes()).unwrap();
        buffered_file.write_all(env_string_name.as_bytes()).unwrap();
    }
}

fn create_switch_database(addr: String) -> Database{
    match Database::create_switch_database(addr){
        Ok(database) => return database,
        Err(err) => panic!("Failed to create database as {:?}", err),
    };
}

fn view_databases(database: &Database) {
    println!("Viewing databases...");
    Database::view_database(&database);
}

fn delete_database(database: &Database) {
    println!("Deleting database...");
    database.delete_database();
}

fn delete_from_database(database: &Database) {
    println!("Enter data you want to delete (if it exists)");
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

    println!("Deleting from database...");
    database.delete_from_database(data);
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