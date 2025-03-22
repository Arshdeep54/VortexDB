// use crate::database::db::Database;
use core::f32;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::database::db::Database;

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

pub fn get_knn(
    database: &mut Database,
    input: Vec<f32>,
    kvalue: usize,
    knn_type: KNNType,
) -> Vec<String> {
    //Finding the first k elements to insert into the BinaryHeap
    let k_nodes = database.tree.traversal(kvalue);
    let mut insert_heap: BinaryHeap<DataHeap> = BinaryHeap::new();
    for node in &k_nodes {
        insert_heap.push(DataHeap {
            key: node.0.clone(),
            distance: distance(input.clone(), node.1.clone(), knn_type),
        });
    }
    let binding = database.tree._root.as_ref().unwrap();
    let (heap, n_visited) = binding.find_nearest_neighbors(input, knn_type, &mut insert_heap);
    let mut ret_vec: Vec<String> = Vec::new();
    ret_vec.push(format!("Visited {} nodes", n_visited));
    for point in heap.iter() {
        ret_vec.push(point.key.clone());
    }
    return ret_vec;
}

pub trait Indexer {
    fn new() -> Self;
    fn add_node(&mut self, data: (String, Vec<f32>), depth: usize);
    fn delete_node(&mut self, data: String);
    fn print_tree_for_debug(&self);

    // Functions for communicating with vectoriser and database
}

pub trait Node {
    // Getter methods to ensure Node contains the necessary information
    fn left(&self) -> Option<&dyn Node>;
    fn right(&self) -> Option<&dyn Node>;
    fn key(&self) -> &str;
    fn vector(&self) -> &Vec<f32>;

    fn find_nearest_neighbors<'a>(
        &'a self,
        input: Vec<f32>,
        knn_type: KNNType,
        heap: &'a mut BinaryHeap<DataHeap>,
    ) -> (&'a mut BinaryHeap<DataHeap>, usize);
}
