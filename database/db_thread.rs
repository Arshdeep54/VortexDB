use std::fs::OpenOptions;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;

const PIPE_PATH : &str = "tmp/db_pipe";

fn write_to_named_pipe(pipe_path: &str, message: &str) -> std::io::Result<()> {
    let mut pipe = OpenOptions::new()
        .write(true)
        .open(pipe_path)?;

    pipe.write_all(message.as_bytes())?;
    pipe.flush()?;
    Ok(())
}

pub fn add_node_pipe(data: (String, Vec<f32>), depth: usize) -> std::io::Result<()> {
    let message = format!("add_node {} {}", data.0, depth);
    if let Err(e) = write_to_named_pipe(PIPE_PATH, &message) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }
    Ok(())
}

pub fn delete_node_pipe(data: String) -> std::io::Result<()> {
    let message = format!("delete_node {}", data);
    if let Err(e) = write_to_named_pipe(PIPE_PATH, &message) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }
    Ok(())
}

pub fn print_tree_debug_pipe() -> std::io::Result<()> {
    let message = "print_tree".to_string();
    if let Err(e) = write_to_named_pipe(PIPE_PATH, &message) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }
    Ok(())
}