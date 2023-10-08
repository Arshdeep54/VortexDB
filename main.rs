use rocksdb::{DB, Options, WriteBatch, WriteOptions};

let path = "/home/hashkat/Documents/sdslabs/VectorDB/rocksdb";

{
    let db = DB::open_default(path).unwrap();
    db.put(b"Some random key", b"Some random value").unwrap();

    match db.get(b"Some random key") {
        Ok(Some(value)) => println!("retrieved value {}", value.to_utf8().unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"Some random key").unwrap();
}

// let _ = DB::destroy(&Options::default(), path);