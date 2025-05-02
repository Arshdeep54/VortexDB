// use crate::database::db::Database;
use crate::indexer::proto::indexer_thread::{Command, KnnType as ProtoKnnType};
use core::f32;
use prost::Message;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::OpenOptions;
use std::io::{BufReader, Read};

const PIPE_PATH: &str = "tmp/db_pipe";

#[derive(Clone, Copy)]
pub enum KNNType {
    Euclidean,
    Manhattan,
    Hamming,
    Cosine,
}

pub struct DataHeap {
    pub key: String,
    pub distance: f32,
}

// These traits must be implemented for a custom BinaryHeap
impl Eq for DataHeap {}
impl Ord for DataHeap {
    fn cmp(&self, other: &DataHeap) -> Ordering {
        if self.distance < other.distance {
            Ordering::Less
        } else if self.distance > other.distance {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}
impl PartialEq for DataHeap {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl PartialOrd for DataHeap {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn distance(a: Vec<f32>, b: Vec<f32>, dist_type: KNNType) -> f32 {
    assert_eq!(a.len(), b.len());
    match dist_type {
        KNNType::Euclidean => {
            let score: Vec<f32> = a
                .iter()
                .zip(b.iter())
                .map(|(&x, &y)| (x - y) * (x - y))
                .collect();
            return score.iter().sum::<f32>().sqrt();
        }
        KNNType::Manhattan => {
            let score: Vec<f32> = a
                .iter()
                .zip(b.iter())
                .map(|(&x, &y)| (x - y).abs())
                .collect();
            return score.iter().sum::<f32>();
        }
        KNNType::Hamming => {
            let score: Vec<f32> = a
                .iter()
                .zip(b.iter())
                .map(|(&x, &y)| (if x != y { 1f32 } else { 0f32 }))
                .collect();
            return score.iter().sum::<f32>();
        }
        KNNType::Cosine => {
            let p_score: Vec<f32> = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).collect();
            let p = p_score.iter().sum::<f32>();
            let q_score: Vec<f32> = a.iter().map(|&n| n * n).collect();
            let q = q_score.iter().sum::<f32>().sqrt();
            let r_score: Vec<f32> = b.iter().map(|&n| n * n).collect();
            let r = r_score.iter().sum::<f32>().sqrt();
            return p / (q * r);
        }
    };
}

pub trait Indexer {
    fn new() -> Self
    where
        Self: Sized;
    fn add_node(&mut self, data: (String, Vec<f32>), depth: usize);
    fn delete_node(&mut self, data: String);
    fn print_tree_for_debug(&self);
    fn get_knn(&self, knn_type: KNNType, k_value: usize, vector: Vec<f32>);
    fn _root(&self) -> Option<&dyn Node>;

    // Function for communicating with vectoriser and database using protobufs
    fn db_thread(&mut self) {
        let pipe = match OpenOptions::new().read(true).open(PIPE_PATH) {
            Ok(pipe) => pipe,
            Err(e) => {
                eprintln!("Failed to open pipe: {}", e);
                return;
            }
        };

        let mut reader = BufReader::new(pipe);
        let mut buffer = Vec::new();
        let mut size_buf = [0u8; 4]; // For reading message size (uint32)

        loop {
            // Read message size (4 bytes for uint32)
            match reader.read_exact(&mut size_buf) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to read message size: {}", e);
                    break;
                }
            }

            // Convert bytes to u32 (size of the message)
            let msg_size = u32::from_le_bytes(size_buf) as usize;

            // Prepare buffer
            buffer.clear();
            buffer.resize(msg_size, 0);

            // Read the actual message
            match reader.read_exact(&mut buffer) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to read message: {}", e);
                    break;
                }
            }

            // Deserialize the protobuf message
            match Command::decode(&buffer[..]) {
                Ok(command) => {
                    match command.command {
                        Some(crate::indexer::proto::indexer_thread::command::Command::AddNode(
                            add_node,
                        )) => {
                            if let Some(vector) = add_node.vector {
                                let depth = add_node.depth as usize;
                                let key = add_node.key;
                                let values = vector.values;
                                self.add_node((key, values), depth);
                            }
                        }
                        Some(
                            crate::indexer::proto::indexer_thread::command::Command::DeleteNode(
                                delete_node,
                            ),
                        ) => {
                            let key = delete_node.key;
                            self.delete_node(key);
                        }
                        Some(crate::indexer::proto::indexer_thread::command::Command::GetKnn(
                            get_knn,
                        )) => {
                            if let Some(vector) = get_knn.vector {
                                let k_value = get_knn.k_value as usize;
                                let values = vector.values;

                                // Convert protobuf enum to our enum
                                let knn_type = match ProtoKnnType::try_from(get_knn.knn_type as i32)
                                {
                                    Ok(ProtoKnnType::Euclidean) => KNNType::Euclidean,
                                    Ok(ProtoKnnType::Manhattan) => KNNType::Manhattan,
                                    Ok(ProtoKnnType::Hamming) => KNNType::Hamming,
                                    Ok(ProtoKnnType::Cosine) => KNNType::Cosine,
                                    _ => {
                                        eprintln!("Unknown KNN type from protobuf");
                                        continue;
                                    }
                                };

                                self.get_knn(knn_type, k_value, values);
                            }
                        }
                        Some(
                            crate::indexer::proto::indexer_thread::command::Command::PrintTree(_),
                        ) => {
                            self.print_tree_for_debug();
                        }
                        None => {
                            eprintln!("Received empty command");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to decode protobuf message: {}", e);
                }
            }
        }
    }
}

pub trait Node {
    // Getter methods to ensure Node contains the necessary information
    fn _left(&self) -> Option<&dyn Node>;
    fn _right(&self) -> Option<&dyn Node>;
    fn _key(&self) -> &str;
    fn _vector(&self) -> &Vec<f32>;

    fn find_nearest_neighbors<'a>(
        &'a self,
        input: Vec<f32>,
        knn_type: KNNType,
        heap: &'a mut BinaryHeap<DataHeap>,
    ) -> (&'a mut BinaryHeap<DataHeap>, usize);
}
