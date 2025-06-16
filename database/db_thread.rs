use crate::indexer::proto::indexer_thread::{
    AddNode,
    Command,
    DeleteNode,
    GetKnn,
    PrintTree,
    Vector,
};
use prost::Message;
use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::Once;
use log::error;

static INIT_PIPE: Once = Once::new();
thread_local! {
    static PIPE_WRITER: RefCell<Option<std::fs::File>> = RefCell::new(None);
}

fn get_pipe_writer() -> std::io::Result<std::fs::File> {
    INIT_PIPE.call_once(|| {
        // Make sure pipe exists
        let _ = crate::indexer::indexing::ensure_pipe_exists();
    });

    let mut result = None;
    PIPE_WRITER.with(|pipe| {
        if pipe.borrow().is_none() {
            *pipe.borrow_mut() = Some(
                OpenOptions::new()
                    .write(true)
                    .open(PIPE_PATH)
                    .expect("Failed to open named pipe for writing:")
            );
        }
        result = Some(pipe.borrow().as_ref().unwrap().try_clone());
    });

    Ok(result.unwrap().expect("Failed to clone pipe writer"))
}

// This module handles the named pipe communication between database and indexer.
const PIPE_PATH: &str = "/tmp/db_pipe";

fn write_protobuf_to_pipe<T: Message>(message: &T) -> std::io::Result<()> {
    let mut pipe = get_pipe_writer().expect("Failed to get pipe writer");

    // Serialize the protobuf message
    let mut buf = Vec::new();
    message.encode(&mut buf).expect("Failed to encode protobuf message");

    let msg_size = buf.len() as u32;
    let size_bytes = msg_size.to_le_bytes();

    pipe.write_all(&size_bytes)?;
    pipe.write_all(&buf)?;
    pipe.flush()?;

    Ok(())
}

pub fn add_node_pipe(data: (String, Vec<f32>), depth: usize) -> std::io::Result<()> {
    init_module_logger!("database");
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
    if let Err(e) = write_protobuf_to_pipe(&command) {
        error!("Failed to write to named pipe: {}", e);
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
            crate::indexer::proto::indexer_thread::command::Command::DeleteNode(delete_node)
        ),
    };

    // Write the command to the pipe
    if let Err(e) = write_protobuf_to_pipe(&command) {
        error!("Failed to write to named pipe: {}", e);
        return Err(e);
    }

    Ok(())
}

pub fn get_knn_pipe(knn_type: u8, k_value: usize, vector_data: Vec<f32>) -> std::io::Result<()> {
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
    if let Err(e) = write_protobuf_to_pipe(&command) {
        error!("Failed to write to named pipe: {}", e);
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
            crate::indexer::proto::indexer_thread::command::Command::PrintTree(print_tree)
        ),
    };

    // Write the command to the pipe
    if let Err(e) = write_protobuf_to_pipe(&command) {
        error!("Failed to write to named pipe: {}", e);
        return Err(e);
    }

    Ok(())
}
