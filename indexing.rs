use crate::db::{deserialize, Database};
use crate::kd_tree::{KDTree, KDTreeNode};
use crate::types::{Data, DataType, VectorData};
use core::f32;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use rocksdb::IteratorMode;

#[derive(Clone, Copy)]
pub enum KNNType {
    Euclidean,
    Manhattan,
    Hamming,
    Cosine,
}

struct DataHeap {
    data: Box<Data>,
    distance: f32,
}

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

impl Data {
    fn distance(&self, other: &Data, dist_type: KNNType) -> f32 {
        assert_eq!(self.vector.vector.len(), other.vector.vector.len());
        //Checks if the embedding types and data types are the same
        // assert_eq!(self.vector.embedding_type, other.vector.embedding_type);
        // assert_eq!(self.data_type, other.data_type);
        match dist_type {
            KNNType::Euclidean => {
                let score: Vec<f32> = self
                    .vector
                    .vector
                    .iter()
                    .zip(other.vector.vector.iter())
                    .map(|(&x, &y)| (x - y) * (x - y))
                    .collect();
                return score.iter().sum::<f32>().sqrt();
            }
            KNNType::Manhattan => {
                let score: Vec<f32> = self
                    .vector
                    .vector
                    .iter()
                    .zip(other.vector.vector.iter())
                    .map(|(&x, &y)| (x - y).abs())
                    .collect();
                return score.iter().sum::<f32>();
            }
            KNNType::Hamming => {
                let score: Vec<f32> = self
                    .vector
                    .vector
                    .iter()
                    .zip(other.vector.vector.iter())
                    .map(|(&x, &y)| (if x != y { 1f32 } else { 0f32 }))
                    .collect();
                return score.iter().sum::<f32>();
            }
            KNNType::Cosine => {
                let a_score: Vec<f32> = self
                    .vector
                    .vector
                    .iter()
                    .zip(other.vector.vector.iter())
                    .map(|(&x, &y)| x * y)
                    .collect();
                let a = a_score.iter().sum::<f32>();
                let b_score: Vec<f32> = self.vector.vector.iter().map(|&n| n * n).collect();
                let b = b_score.iter().sum::<f32>().sqrt();
                let c_score: Vec<f32> = other.vector.vector.iter().map(|&n| n * n).collect();
                let c = c_score.iter().sum::<f32>().sqrt();
                return a / (b * c);
            }
        };
    }
}

pub fn get_knn(
    database: &Database,
    input: &Vec<f32>,
    kvalue: usize,
    knn_type: KNNType,
) -> Vec<String> {
    // Pending  - add the newly created tree to the database
    //          - k nearest neighbors instead of nearest neighbor

    let mut tree = KDTree::new();

    let iter = database.db.iterator(IteratorMode::Start);

    for item in iter {
        let (_, value) = item.unwrap();
        let data = deserialize(&value);
        if tree.dim == 0 {
            tree.dim = data.vector.vector.len();
        }
        assert_eq!(tree.dim, data.vector.vector.len());
        tree.add_node(data, 0);
    }

    // Converting the input vector into a Data struct
    let input_data = Data {
        vector: VectorData {
            vector: input.clone(),
            embedding_type: String::new(),
        },
        data_type: DataType::Blob,
        payload: "Input Vector".to_string(),
    };

    let binding = tree._root.unwrap();
    let (point, n_visited) = binding
        .as_ref()
        .find_nearest_neighbor(&input_data, knn_type);

    let mut ret_vec: Vec<String> = Vec::new();
    ret_vec.push(format!("Nearest Neighbor: {:?}", point));
    ret_vec.push(format!(
        "Distance: {}",
        binding.as_ref().dataset.distance(&point, knn_type)
    ));
    ret_vec.push(format!("Nodes visited: {}", n_visited));
    return ret_vec;
}

impl KDTreeNode {
    pub fn find_nearest_neighbor<'a>(
        &'a self,
        point: &Data,
        knn_type: KNNType,
        k_value: usize,
    ) -> (&'a Data, usize) {
        let mut heap: BinaryHeap<DataHeap> = BinaryHeap::new();
        heap.push(DataHeap {
            data: Box::new(self.dataset),
            distance: self.dataset.distance(point, knn_type),
        });
        self.find_nearest_neighbor_helper(point, 1, knn_type, &heap, k_value)
    }

    fn find_nearest_neighbor_helper<'a>(
        &'a self,
        point: &Data,
        n_visited: usize,
        knn_type: KNNType,
        distances: &BinaryHeap<DataHeap>,
        k_value: usize,
    ) -> (&'a Data, usize) {
        if distances.is_empty() {
            panic!("Empty heap entered!");
        }

        let mut my_best = distances.peek().unwrap().data;
        let mut my_best_dist = distances.peek().unwrap().distance;
        let mut my_n_visited = n_visited;
        let mut my_distances = distances;

        if my_distances.len() < k_value {
            
        }

        if self.dataset.vector.vector[self.dim] < point.vector.vector[self.dim]
            && self.right.is_some()
        {
            let (a, b) = self.left.as_ref().unwrap().find_nearest_neighbor_helper(
                point,
                my_best,
                my_best_dist,
                my_n_visited,
                knn_type,
            );
            my_best = a;
            my_n_visited = b;
        }

        // distance along this node's axis
        let axis_dist = self.dataset.distance(point, knn_type);
        if axis_dist <= my_best_dist {
            // self can only be nearer than best if axis_dist is less than
            // best_dist because axis_dist is a lower bound for
            // self_dist
            let self_dist = self.dataset.distance(point, knn_type.clone());
            if self_dist < my_best_dist {
                my_best = &self.dataset;
                my_best_dist = self_dist;
            }

            // bookkeeping
            my_n_visited += 1;

            // same reasoning applies for the far side of the split
            if self.dataset.vector.vector[self.dim] < point.vector.vector[self.dim]
                && self.left.is_some()
            {
                let (a, b) = self.left.as_ref().unwrap().find_nearest_neighbor_helper(
                    point,
                    my_best,
                    my_best_dist,
                    my_n_visited,
                    knn_type,
                );
                my_best = a;
                my_n_visited = b;
            } else if self.right.is_some() {
                let (a, b) = self.right.as_ref().unwrap().find_nearest_neighbor_helper(
                    point,
                    my_best,
                    my_best_dist,
                    my_n_visited,
                    knn_type,
                );
                my_best = a;
                my_n_visited = b;
            }
        }

        (my_best, my_n_visited)
    }

    fn test_fn() {
        let mut heap: BinaryHeap<DataHeap> = BinaryHeap::new();
    }
}

//check everything works
//work on remaining functions
//integrate them all

//add a binary heap to the knn helper function, fill in the heap till its of k size, then check for the maximum value of k like
