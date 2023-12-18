mod db;
mod types;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::{env, io, io::Write};
// use std::ops::Deref;
use db::{check_database, check_path, find_databases, Database};
use types::{Data, DataType, VectorData};
mod vectoriser;

fn main() {
    println!("Welcome");

    let mut databases = find_databases();

    loop {
        println!("\n(1) Show databases");
        println!("(2) Use database");
        println!("(3) Delete database");
        println!("(0) Exit");

        let mut select = String::new();
        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        match select.trim() {
            "1" => {
                println!("\nAvaiable databases are...\n");
                for (key, _) in databases.iter() {
                    println!("{}", key);
                }
            }
            "2" => {
                println!("Enter name of database");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Faile to read line");

                if databases.contains_key(input.trim()) {
                    let mut database: Option<Database> =
                        valid_database(&mut databases, input.trim());

                    if database.is_none() {
                        let exit = change_path(&mut databases, input.trim());
                        if exit {
                            databases.remove(input.trim());
                            continue;
                        } else {
                            database = valid_database(&mut databases, input.trim());
                        }
                    }

                    let database: Database = database.unwrap();
                    loop {
                        //delete needs to be implmented

                        println!("{}", input.trim());
                        println!("(1) Insert in Database");
                        println!("(2) View Database");
                        println!("(3) Get from Database");
                        println!("(4) Delete from Database");
                        println!("(5) View current path");
                        println!("(6) Switch database");

                        let mut choice = String::new();
                        io::stdin()
                            .read_line(&mut choice)
                            .expect("Failed to read line");

                        match choice.trim() {
                            "1" => {
                                insert_in_database(&database);
                            }
                            "2" => {
                                view_databases(&database);
                            }
                            "3" => {
                                get_from_database(&database);
                            }
                            "4" => {
                                delete_from_database(&database);
                            }
                            "5" => {
                                Database::view_current_path(&database);
                            }
                            "6" => {
                                break;
                            }
                            _ => println!("Invalid choice"),
                        }
                    }
                    write_env(&databases);
                } else {
                    println!("Invalid input");
                }
            }
            "3" => {
                println!("Enter the database name");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Filed to read line");

                if databases.contains_key(input.trim()) {
                    let exit = change_path(&mut databases, &input);
                    if exit {
                        continue;
                    }

                    let database = valid_database(&mut databases, input.trim()).unwrap();
                    delete_database(&database);
                } else {
                    println!("Invalid input");
                }
            }
            "0" => {
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        };
    }
}

fn valid_database(databases: &mut HashMap<String, String>, input: &str) -> Option<Database> {
    let file_path = databases.get(input).unwrap();
    let mut database: Option<Database> = None;
    if check_path(file_path) {
        println!("Path is valid, validating database...");
        if check_database(file_path) {
            println!("Database exists on current path");
            let new_path = (*file_path).clone();
            database = Some(create_switch_database(new_path));
        } else {
            println!("Database does not exist on current path.\n");
            println!("Creating a new database...");
            let new_path = (*file_path).clone();
            database = Some(create_switch_database(new_path));
        }
    } else {
        println!("Path in .env file is invalid");
    }

    return database;
}

fn change_path(databases: &mut HashMap<String, String>, input: &str) -> bool {
    let mut br = false;

    loop {
        println!("Enter a new path for database");
        println!("(1) Use current working directory");
        println!("(2) Enter custom path");
        println!("(3) Remove database");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim() {
            "1" => {
                let current_dir = env::current_dir().unwrap();
                let absolute_path = current_dir.canonicalize().unwrap();
                println!("Setting path to: {}", absolute_path.display());
                let absolute_path = absolute_path.as_os_str().to_str().unwrap().to_string();
                databases.insert(input.to_string(), absolute_path.trim().to_string());
                write_env(databases);
                break;
            }
            "2" => {
                println!("Enter path");
                let mut new_path = String::new();
                io::stdin()
                    .read_line(&mut new_path)
                    .expect("Failed to read line");
                if check_path(&new_path.trim().to_string()) {
                    databases.insert(input.to_string(), new_path.trim().to_string());
                    write_env(databases);
                    break;
                } else {
                    println!("Please enter a valid path");
                }
            }
            "3" => {
                br = true;
                break;
            }
            _ => {
                println!("Invalid input\n\n");
            }
        };
    }
    return br;
}

fn write_env(databases: &HashMap<String, String>) {
    let file_path = ".env";
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut buffered_file = io::BufWriter::new(file);

    let env_string = format!("NUMBER_OF_DATABASE={}\n", databases.len());
    buffered_file.write_all(env_string.as_bytes()).unwrap();

    let mut count = 0;

    for (key, value) in databases.iter() {
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

fn create_switch_database(addr: String) -> Database {
    match Database::create_switch_database(addr) {
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
    println!("(1) Delete by entering data");
    println!("(2) Delete by entering key");

    let mut select = String::new();
    io::stdin()
        .read_line(&mut select)
        .expect("Failed to read line");

    match select.trim() {
        "1" => {
            println!("Enter data you want to delete (if it exists)");
            println!("Enter data type (Text, Image, Audio, Blob): ");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
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
            io::stdin()
                .read_line(&mut payload)
                .expect("Failed to read line");

            let data = vectoriser::vectorize(Data {
                vector: VectorData::default(),
                payload: payload,
                data_type: datatype,
            });

            println!("Deleting from database...");
            database.delete_from_database(data);
        }
        "2" => {
            println!("Enter key");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            database.delete_from_database_with_key(input.trim());
        }
        _ => {
            println!("Invalid choice");
            return;
        }
    }
}

fn insert_in_database(database: &Database) {
    println!("Enter data type (Text, Image, Audio, Blob): ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
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
    io::stdin()
        .read_line(&mut payload)
        .expect("Failed to read line");

    let data = vectoriser::vectorize(Data {
        vector: VectorData::default(),
        payload: payload,
        data_type: datatype,
    });

    println!("Inserting into a database...");
    database.insert_in_database(data);
}

fn get_from_database(database: &Database) {
    println!("Enter key of data");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read key");
    database.get(input.trim());
}
