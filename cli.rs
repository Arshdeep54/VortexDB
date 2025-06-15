use crate::database::{db, db_thread, dbpath, keygen, types};
use crate::indexer::indexing::{self, ensure_pipe_exists, Indexer};
use crate::indexer::indexing_models::kd_tree::KDTree;
use crate::vectorisers::vectoriser;
use db::Database;
use dbpath::{check_database, check_path, find_databases, write_env};
use indexing::KNNType;
use keygen::deserialize;
use std::collections::HashMap;
use std::thread;
use std::{env, io};
use types::{Data, DataType, VectorData};

use rocksdb::IteratorMode;

fn clear_screen() {
    // Add a small delay for better user experience
    std::thread::sleep(std::time::Duration::from_millis(500));

    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("clear").status();
    }
}

pub fn run_cli() {
    clear_screen();
    println!("Welcome to Vector DB");
    clear_screen();
    main_menu();
}

fn main_menu() {
    let mut databases = find_databases();

    loop {
        println!("(1) Show databases");
        println!("(2) Use database");
        println!("(3) Add new database");
        println!("(4) Delete database");
        println!("(0) Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                clear_screen();
                show_databases(&mut databases);
            }
            "2" => {
                clear_screen();
                use_databases(&mut databases);
            }
            "3" => {
                clear_screen();
                add_databases(&mut databases);
            }
            "4" => {
                clear_screen();
                delete_databases(&mut databases);
            }
            "0" => {
                break;
            }
            _ => {
                println!("Invalid choice");
                clear_screen();
            }
        };
    }
}

fn show_databases(databases: &mut HashMap<String, String>) {
    println!("Avaiable databases are...");
    for (key, _) in databases.iter() {
        println!("{}", key);
    }
}

fn use_databases(databases: &mut HashMap<String, String>) {
    println!("Enter name of database");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Faile to read line");

    if !databases.contains_key(input.trim()) {
        println!("Database does not exist!");
        return;
    }

    let mut database: Option<Database> = valid_database(databases, input.trim());

    if database.is_none() {
        let exit = change_path(databases, input.trim());
        if exit {
            databases.remove(input.trim());
            return;
        } else {
            database = valid_database(databases, input.trim());
        }
    }

    loop {
        println!("Please select an indexer");
        println!("(1) KDTree");
        println!("(2) BallTree");
        println!("(3) Annoy");
        println!("(4) HNSW");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if let Err(err) = ensure_pipe_exists() {
            println!("Pipe does not exist: {:?}", err);
            return;
        }

        match choice.trim() {
            "1" => {
                let mut kdtree = KDTree::new();
                thread::spawn(move || {
                    kdtree.db_thread();
                });
                println!("KDTree is running in the background");
                break;
            }
            "2" => {
                println!("BallTree is not implemented yet.");
            }
            "3" => {
                println!("Annoy is not implemented yet.");
            }
            "4" => {
                println!("HNSW is not implemented yet.");
            }
            _ => {
                println!("Invalid choice. Please select a valid option.");
            }
        }
    }

    let database = &mut database.unwrap();

    database.sync_with_indexer();

    loop {
        println!("{}", input.trim());
        println!("(1) Insert in Database");
        println!("(2) View Database");
        println!("(3) Get from Database");
        println!("(4) Delete from Database");
        println!("(5) Find K Nearest Neighbours");
        println!("(6) View current path");
        println!("(7) Print indexer tree");
        println!("(8) Switch database");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                insert_in_database(database);
            }
            "2" => {
                view_database(database);
            }
            "3" => {
                get_from_database(database);
            }
            "4" => {
                delete_from_database(database);
            }
            "5" => {
                find_knn(database);
            }
            "6" => {
                view_current_path(database);
            }
            "7" => {
                if let Err(e) = db_thread::print_tree_debug_pipe() {
                    println!("Failed to print tree: {}", e);
                }
            }
            "8" => {
                // TODO: Close the thread opened by indexer
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn add_databases(databases: &mut HashMap<String, String>) {
    println!("Enter the database name");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    if databases.contains_key(name.trim()) {
        println!("This name already exists");
        return;
    }
    if !change_path(databases, name.trim()) {
        println!("Created database successfully");
    }
}

fn delete_databases(databases: &mut HashMap<String, String>) {
    println!("Enter the database name");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if databases.contains_key(input.trim()) {
        let exit = change_path(databases, &input);
        if exit {
            return;
        }

        let database = valid_database(databases, input.trim()).unwrap();
        delete_database(&database);
    } else {
        println!("Database does not exist");
    }
}

fn valid_database(databases: &mut HashMap<String, String>, input: &str) -> Option<Database> {
    //TODO: If path does not exist then create the path
    let file_path = databases.get(input).unwrap();
    let mut database: Option<Database> = None;
    if check_path(file_path) {
        println!("Path is valid, validating database...");
        if check_database(file_path) {
            println!("Database exists on current path");

            match Database::open_database(input, file_path) {
                Ok(db) => {
                    database = Some(db);
                }
                Err(err) => panic!("Failed to create database as {:?}", err),
            };
        } else {
            println!("Database does not exist on current path.");
            println!("Creating a new database...");

            match Database::create_database(input, file_path) {
                Ok(db) => {
                    database = Some(db);
                }
                Err(err) => panic!("Failed to create database as {:?}", err),
            };
        }
    } else {
        println!("Path in .env file is invalid");
    }

    database
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
                println!("Invalid input");
            }
        };
    }
    br
}

fn delete_database(database: &Database) {
    println!("Deleting database...");
    match database.delete_database() {
        Ok(()) => println!("Database deleted successfully"),
        Err(e) => println!("{}", e),
    };
}

fn insert_in_database(database: &mut Database) {
    let Some(data) = read_data() else { return };
    println!("Inserting into a database...");
    match database.insert_in_database(data) {
        Ok(key) => {
            println!("Inserted with key {}", key);
        }
        Err(e) => println!("{}", e),
    };
}

fn view_database(database: &Database) {
    let iter = database.db.iterator(IteratorMode::Start); //iterates from the start
    println!("Iterating over database...");
    for item in iter {
        let (key, value) = item.unwrap();
        let hex_strings: Vec<String> = key.iter().map(|b| format!("{:02x}", b)).collect();
        let result = hex_strings.join("");
        let vec = deserialize(&value);
        println!("Key: {:?}\nValue: {:?}", result, vec.payload);
    }
}

fn get_from_database(database: &Database) {
    println!("Enter key of data");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read key");
    match database.get_data_from_key(input.trim()) {
        Ok(v) => match v {
            Ok(Some(v)) => println!("{:?}", v.payload),
            Ok(None) => {
                println!("Key not found");
            }
            Err(e) => {
                println!("{}", e);
            }
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn delete_from_database(database: &mut Database) {
    println!("(1) Delete by entering data");
    println!("(2) Delete by entering key");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    match choice.trim() {
        "1" => {
            let Some(data) = read_data() else { return };
            match database.delete_from_database_with_value(data) {
                Ok(v) => match v {
                    Some(_) => {
                        println!("Data deleted successfully");
                    }
                    None => println!("Key not found"),
                },
                Err(e) => println!("{}", e),
            };
        }
        "2" => {
            println!("Enter key");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match database.delete_from_database_with_key(input.trim()) {
                Ok(v) => match v {
                    Ok(Some(_)) => {
                        println!("Data deleted successfully");
                    }
                    Ok(None) => println!("Key not found"),
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            };
        }
        _ => {
            println!("Invalid choice");
        }
    }
}

fn read_data() -> Option<Data> {
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
            println!("Invalid data type");
            return None;
        }
    };
    println!("Enter payload: ");
    let mut payload = String::new();
    io::stdin()
        .read_line(&mut payload)
        .expect("Failed to read line");

    let mut embedding_type: String = String::new();
    println!("Enter embedding type: ");
    io::stdin()
        .read_line(&mut embedding_type)
        .expect("Failed to read line");

    let vec = vectoriser::vectorise(&payload, "");

    let data: Data = Data {
        vector: VectorData {
            vector: vec.vector,
            embedding_type,
        },
        payload,
        data_type: datatype,
    };

    println!("Your Data is");
    println!("{:?}", data.payload);
    println!("(1) Confirm");
    println!("(0) Cancel");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => Some(data),
        _ => None,
    }
}

fn find_knn(database: &mut Database) {
    println!("Please Select input for KNN");
    println!("(1) Enter Data");
    println!("(2) Enter Key");
    let givenvec;
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    match choice.trim() {
        "1" => {
            let mut payload = String::new();
            io::stdin()
                .read_line(&mut payload)
                .expect("Failed to read line");
            let vec = vectoriser::vectorise(&payload, "");
            givenvec = vec.vector;
            println!("{:?}", givenvec)
        }
        "2" => {
            println!("Enter key");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            match database.get_data_from_key(input.trim()) {
                Ok(v) => match v {
                    Ok(Some(v)) => {
                        givenvec = v.vector.vector;
                    }
                    Ok(None) => {
                        println!("Key not found");
                        return;
                    }
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            }
        }
        _ => {
            println!("Invalid choice");
            return;
        }
    }

    println!("Please Enter K Value");

    let mut kvalue = String::new();
    io::stdin()
        .read_line(&mut kvalue)
        .expect("Failed to read line");
    let kvalue: usize = kvalue.trim().parse().unwrap();

    loop {
        println!("Please Select method for KNN");
        println!("(1) Euclidean Distance");
        println!("(2) Manhattan Distance");
        println!("(3) Hamming Distance");
        println!("(4) Cosine Similarity");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim() {
            "1" => {
                let result = database.get_knn(KNNType::Euclidean as u8, kvalue, givenvec);
                if let Ok(v) = result {
                    println!("Result: {:?}", v);
                } else {
                    println!("Error: {:?}", result);
                }
                break;
            }
            "2" => {
                let result = database.get_knn(KNNType::Manhattan as u8, kvalue, givenvec);
                if let Ok(v) = result {
                    println!("Result: {:?}", v);
                } else {
                    println!("Error: {:?}", result);
                }
                break;
            }
            "3" => {
                let result = database.get_knn(KNNType::Hamming as u8, kvalue, givenvec);
                if let Ok(v) = result {
                    println!("Result: {:?}", v);
                } else {
                    println!("Error: {:?}", result);
                }
                break;
            }
            "4" => {
                let result = database.get_knn(KNNType::Cosine as u8, kvalue, givenvec);
                if let Ok(v) = result {
                    println!("Result: {:?}", v);
                } else {
                    println!("Error: {:?}", result);
                }
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn view_current_path(database: &Database) {
    println!("{}", database.get_current_path());
}
