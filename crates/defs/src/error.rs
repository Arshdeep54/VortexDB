use std::io;

#[derive(Debug)]
pub enum ServerError {
    Bind(io::Error),
    Serve(io::Error),
}

#[derive(Debug)]
pub enum AppError {
    ServerError(ServerError),
}

// Error type for server
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
