// use crate::database::db::Database;
use core::f32;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

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

    // Functions for communicating with vectoriser and database
    fn db_thread(&mut self) {
        let pipe = match OpenOptions::new().read(true).open(PIPE_PATH) {
            Ok(pipe) => pipe,
            Err(e) => {
                eprintln!("Failed to open pipe: {}", e);
                return;
            }
        };

        let reader = BufReader::new(pipe);
        for line in reader.lines() {
            match line {
                Ok(command) => {
                    // Process the command here
                    println!("Received command: {}", command);

                    match command.split_whitespace().collect::<Vec<&str>>().as_slice() {
                        ["add_node", key, depth, ..] => {
                            let depth: usize = depth.parse().unwrap_or(0);
                            let vector: Vec<f32> = command
                                .split_whitespace()
                                .skip(3)
                                .map(|x| x.parse().unwrap_or(0.0))
                                .collect();
                            self.add_node((key.to_string(), vector), depth);
                        }
                        ["delete_node", key] => {
                            self.delete_node(key.to_string());
                        }
                        ["print_tree"] => {
                            self.print_tree_for_debug();
                        }
                        ["get_knn", knn_type, k_value, ..] => {
                            let knn_type = match *knn_type {
                                "euclidean" => KNNType::Euclidean,
                                "manhattan" => KNNType::Manhattan,
                                "hamming" => KNNType::Hamming,
                                "cosine" => KNNType::Cosine,
                                _ => {
                                    eprintln!("Unknown KNN type: {}", knn_type);
                                    continue;
                                }
                            };
                            let k_value: usize = k_value.parse().unwrap_or(0);
                            let vector: Vec<f32> = command
                                .split_whitespace()
                                .skip(3)
                                .map(|x| x.parse().unwrap_or(0.0))
                                .collect();
                            self.get_knn(knn_type, k_value, vector);
                        }
                        _ => {
                            eprintln!("Unknown command: {}", command);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read line from pipe: {}", e);
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
