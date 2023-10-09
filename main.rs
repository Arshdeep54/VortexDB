use rocksdb::{DB};

fn main(){
    let path = "/home/hashkat/Documents/sdslabs/VectorDB/rocksdb";
    let db = DB::open_default(path).unwrap();
    db.put(b"Some random key", b"Some random value").unwrap();

    match db.get(b"Some random key") {
        Ok(Some(value)) => {
            if let Ok(value_as_str) = std::str::from_utf8(&value) {
                println!("retrieved value {}", value_as_str);
            } else {
                println!("Invalid UTF-8 data");
            }
        },
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"Some random key").unwrap();
}

// let _ = DB::destroy(&Options::default(), path);