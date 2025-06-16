use log::{ debug, error, info, warn };

use reqwest::blocking::Client;
use serde_derive::{ Deserialize, Serialize };

// use std::collections::HashMap;
// use std::fs::File;
// use std::io::{BufReader, BufRead};
// use std::os::unix::fs::FileTypeExt;

// const PIPE_PATH: &str = "/tmp/vector_pipe";

#[derive(Serialize)]
struct VectorizationRequest {
    text: String,
    pooling_strategy: String,
}

#[derive(Deserialize)]
pub struct VectorResponse {
    text: String,
    pub vector: Vec<f32>,
}

#[cfg(feature = "test_vectors")]
pub fn vectorise(input: &str, _pooling_strategy: &str) -> VectorResponse {
    init_module_logger!("vectoriser");
    info!("Using test vectoriser with fixed size vectors");
    // Generate a deterministic vector of size 10 based on the input
    let mut hash: u64 = 0;
    for byte in input.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }

    // Create a fixed-size vector of 10 elements
    let vector: Vec<f32> = (0..10)
        .map(|i| {
            // Use the hash and position to generate a deterministic value
            let value = (((hash + (i as u64)) % 100) as f32) / 100.0;
            value
        })
        .collect();

    debug!("Generated test vector: {:?}", vector);

    VectorResponse {
        text: input.to_string(),
        vector,
    }
}

#[cfg(not(feature = "test_vectors"))]
pub fn vectorise(input: &str, pooling_strategy: &str) -> VectorResponse {
    init_module_logger!("vectoriser");
    info!("Sending vectorization request to API");

    let client = Client::new();
    let response = client
        .post("http://localhost:8000/vectorize/")
        .json(
            &(VectorizationRequest {
                text: input.to_string(),
                pooling_strategy: pooling_strategy.to_string(),
            })
        )
        .send();

    match response {
        Ok(resp) => {
            info!("Request sent successfully. Status: {}", resp.status());
            match resp.json::<VectorResponse>() {
                Ok(vector_response) => vector_response,
                Err(e) => {
                    error!("Failed to parse response: {}", e);
                    panic!("Vector response parsing failed");
                }
            }
        }
        Err(e) => {
            error!("Failed to send request: {}", e);
            panic!("Vector request failed");
        }
    }
}

// // Reads from the named pipe and processes the vector
// pub fn read_from_named_pipe() {
//     // Ensure the named pipe exists
//     if !std::path::Path::new(PIPE_PATH).exists() {
//         eprintln!("Named pipe does not exist at {}", PIPE_PATH);
//         return;
//     }

//     // Check if the file is a named pipe
//     let metadata = std::fs::metadata(PIPE_PATH).expect("Unable to fetch metadata for the named pipe");
//     if !metadata.file_type().is_fifo() {
//         eprintln!("The path is not a named pipe");
//         return;
//     }

//     // Open the named pipe for reading
//     let pipe = match File::open(PIPE_PATH) {
//         Ok(file) => file,
//         Err(e) => {
//             eprintln!("Failed to open named pipe: {}", e);
//             return;
//         }
//     };
//     let reader = BufReader::new(pipe);

//     // Continuously read from the pipe
//     println!("Listening for vectors on named pipe: {}", PIPE_PATH);
//     for line in reader.lines() {
//         match line {
//             Ok(json_line) => {
//                 // Parse JSON data from the named pipe
//                 let parsed: Result<HashMap<String, Vec<f32>>, _> = serde_json::from_str(&json_line);
//                 match parsed {
//                     Ok(data) => {
//                         if let Some(vector) = data.get("vector") {
//                             println!("Received vector: {:?}", vector);
//                             // Perform validation
//                             if vector.iter().all(|v| v.is_finite()) {
//                                 println!("Vector is valid!");
//                             } else {
//                                 println!("Invalid vector received!");
//                             }
//                         } else {
//                             println!("No vector found in the data!");
//                         }
//                     }
//                     Err(e) => eprintln!("Failed to parse JSON from pipe: {}", e),
//                 }
//             }
//             Err(e) => eprintln!("Error reading from named pipe: {}", e),
//         }
//     }
// }

// fn main() {
//     // Example: Sending a request and printing the response
//     let input_text = "This is a test sentence";
//     let pooling_strategy = "mean";
//     let response = vectorise(input_text, pooling_strategy);
//     println!("Vector response: {:?}", response.vector);

//     // Reading vector data from the named pipe
//     read_from_named_pipe();
// }
