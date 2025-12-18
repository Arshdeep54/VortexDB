use std::io;
#[derive(Debug)]
pub enum DbError {
    ParseError,
    StorageError(String),
    SerializationError(String),
    DeserializationError,
    IndexError(String),
    LockError,
}

#[derive(Debug)]
pub enum ServerError {
    Bind(io::Error),
    Serve(io::Error),
}

#[derive(Debug)]
pub enum AppError {
    DbError(DbError),
    ServerError(ServerError),
}
