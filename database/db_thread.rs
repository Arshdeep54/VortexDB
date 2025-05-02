use crate::indexer::proto::indexer_thread::{
    AddNode, Command, DeleteNode, GetKnn, PrintTree, Vector,
};

use prost::Message;
use std::fs::OpenOptions; 
use std::io::prelude::*;

// This module handles the named pipe communication between database and indexer.
const PIPE_PATH: &str = "tmp/db_pipe";

fn write_protobuf_to_pipe<T: Message>(pipe_path: &str, message: &T) -> std::io::Result<()> {
    // Serialize the protobuf message
    let mut buf = Vec::new();
    message
        .encode(&mut buf)
        .expect("Failed to encode protobuf message");

    // Get the size of the serialized message
    let msg_size = buf.len() as u32;
    let size_bytes = msg_size.to_le_bytes();

    // Open the pipe for writing
    let mut pipe = OpenOptions::new().write(true).open(pipe_path)?;

    // Write the message size as a 4-byte prefix
    pipe.write_all(&size_bytes)?;

    // Write the actual message
    pipe.write_all(&buf)?;
    pipe.flush()?;

    Ok(())
}

pub fn add_node_pipe(data: (String, Vec<f32>), depth: usize) -> std::io::Result<()> {
    // Create the Vector protobuf message
    let vector = Vector { values: data.1 };

    // Create the AddNode protobuf message
    let add_node = AddNode {
        key: data.0,
        depth: depth as u32,
        vector: Some(vector),
    };

    // Create the Command protobuf message
    let command = Command {
        command: Some(crate::indexer::proto::indexer_thread::command::Command::AddNode(add_node)),
    };

    // Write the command to the pipe
    if let Err(e) = write_protobuf_to_pipe(PIPE_PATH, &command) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }

    Ok(())
}

pub fn delete_node_pipe(data: String) -> std::io::Result<()> {
    // Create the DeleteNode protobuf message
    let delete_node = DeleteNode { key: data };

    // Create the Command protobuf message
    let command = Command {
        command: Some(
            crate::indexer::proto::indexer_thread::command::Command::DeleteNode(delete_node),
        ),
    };

    // Write the command to the pipe
    if let Err(e) = write_protobuf_to_pipe(PIPE_PATH, &command) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }

    Ok(())
}

pub fn get_knn_pipe(
    knn_type: u8,
    k_value: usize,
    vector_data: Vec<f32>,
) -> std::io::Result<()> {
    // Create the Vector protobuf message
    let vector = Vector {
        values: vector_data,
    };

    // Create the GetKNN protobuf message
    let get_knn = GetKnn {
        knn_type: knn_type as i32,
        k_value: k_value as u32,
        vector: Some(vector),
    };

    // Create the Command protobuf message
    let command = Command {
        command: Some(crate::indexer::proto::indexer_thread::command::Command::GetKnn(get_knn)),
    };

    // Write the command to the pipe
    if let Err(e) = write_protobuf_to_pipe(PIPE_PATH, &command) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }

    Ok(())
}

pub fn print_tree_debug_pipe() -> std::io::Result<()> {
    // Create the PrintTree protobuf message
    let print_tree = PrintTree {};

    // Create the Command protobuf message
    let command = Command {
        command: Some(
            crate::indexer::proto::indexer_thread::command::Command::PrintTree(print_tree),
        ),
    };

    // Write the command to the pipe
    if let Err(e) = write_protobuf_to_pipe(PIPE_PATH, &command) {
        eprintln!("Failed to write to named pipe: {}", e);
        return Err(e);
    }

    Ok(())
}
