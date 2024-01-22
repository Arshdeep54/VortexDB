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

pub struct DataHeap {
    data: Box<Data>,
    distance: f32,
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

    //Finding the first k elements to insert into the BinaryHeap
    let k_nodes = tree.traversal(kvalue);
    let mut insert_heap: BinaryHeap<DataHeap> = BinaryHeap::new();
    for node in &k_nodes {
        insert_heap.push(DataHeap {
            data: Box::new(node.clone()),
            distance: node.clone().distance(&input_data, knn_type),
        })
    }

    let binding = tree._root.unwrap();
    let (heap, n_visited) = binding.find_nearest_neighbors(&input_data, knn_type, &mut insert_heap);
    let mut ret_vec: Vec<String> = Vec::new();
    let mut i = 1;
    ret_vec.push(format!("Visited {} nodes", n_visited));
    for point in heap.iter() {
        ret_vec.push(format!("{}. Point: {:?}", i, point.data));
        ret_vec.push(format!("   Distance: {}", point.distance));
        i += 1;
    }
    return ret_vec;
}

impl KDTreeNode {
    pub fn find_nearest_neighbors<'a>(
        &'a self,
        point: &Data,
        knn_type: KNNType,
        heap: &'a mut BinaryHeap<DataHeap>,
    ) -> (&'a mut BinaryHeap<DataHeap>, usize) {
        self.find_nearest_neighbor_helper(point, 1, knn_type, heap)
    }

    fn find_nearest_neighbor_helper<'a>(
        &'a self,
        point: &Data,
        n_visited: usize,
        knn_type: KNNType,
        distances: &'a mut BinaryHeap<DataHeap>,
    ) -> (&'a mut BinaryHeap<DataHeap>, usize) {
        if distances.is_empty() {
            panic!("Empty heap entered!");
        }

        let mut my_n_visited = n_visited;
        let mut my_distances = distances;

        if self.dataset.vector.vector[self.dim] < point.vector.vector[self.dim]
            && self.right.is_some()
        {
            let (a, b) = self.left.as_ref().unwrap().find_nearest_neighbor_helper(
                point,
                my_n_visited,
                knn_type,
                my_distances,
            );
            my_distances = a;
            my_n_visited = b;
        }

        // distance along this node's axis
        let axis_dist = self.dataset.distance(point, knn_type);
        if axis_dist <= my_distances.peek().unwrap().distance {
            // self can only be nearer than worst if axis_dist is less than worst_dist because axis_dist is a lower bound for self_dist
            let self_dist = self.dataset.distance(point, knn_type.clone());
            if self_dist < my_distances.peek().unwrap().distance {
                my_distances.pop();
                my_distances.push(DataHeap {
                    data: Box::new(self.dataset.clone()),
                    distance: self_dist,
                });
            }

            // bookkeeping
            my_n_visited += 1;

            // same reasoning applies for the far side of the split
            if self.dataset.vector.vector[self.dim] < point.vector.vector[self.dim]
                && self.left.is_some()
            {
                let (a, b) = self.left.as_ref().unwrap().find_nearest_neighbor_helper(
                    point,
                    my_n_visited,
                    knn_type,
                    my_distances,
                );
                my_distances = a;
                my_n_visited = b;
            } else if self.right.is_some() {
                let (a, b) = self.right.as_ref().unwrap().find_nearest_neighbor_helper(
                    point,
                    my_n_visited,
                    knn_type,
                    my_distances,
                );
                my_distances = a;
                my_n_visited = b;
            }
        }

        (my_distances, my_n_visited)
    }
}
