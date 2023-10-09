use rocksdb::{DB, Options, WriteBatch};

fn main(){
    let path = "/home/hashkat/Documents/sdslabs/VectorDB/rocksdb";
    let mut options = Options::default();
    options.increase_parallelism(12);
    options.optimize_level_style_compaction(512 * 1024 * 1024);
    options.create_if_missing(true);


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
    {
        let mut batch = WriteBatch::default();
        batch.put(b"my key", b"my value");
        batch.put(b"key2", b"value2");
        batch.put(b"key3", b"value3");

        // delete_range is supported when use without transaction
        batch.delete_range(b"key2", b"key3");

        db.write(batch).unwrap();
    }
    match db.get(b"my key") {
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