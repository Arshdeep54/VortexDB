use core::{DbError, DenseVector, IndexedVector, PointId, Similarity};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    vec,
};

use crate::{distance, VectorIndex};

pub struct KDTree {
    dim: usize,
    root: Option<Box<KDTreeNode>>,
    // An in memory point map for lookup during delete
    point_map: HashMap<PointId, DenseVector>,
}

// the node which will be the part of the KD Tree
pub struct KDTreeNode {
    indexed_vector: IndexedVector,
    split_dim: usize,
    left: Option<Box<KDTreeNode>>,
    right: Option<Box<KDTreeNode>>,
    is_deleted: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Neighbor {
    id: PointId,
    distance: f32,
}

impl Eq for Neighbor {}

// Custom Ord implementation for the max-heap
impl Ord for Neighbor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Neighbor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl KDTree {
    pub fn mock() {
        //here is the mock code
    }

    // Build an empty index with no points
    pub fn build_empty(dim: usize) -> Self {
        KDTree {
            dim,
            root: None,
            point_map: HashMap::new(),
        }
    }

    // Builds the vector index from provided vectors, there should atleast be single vector for dim calculation
    pub fn build(mut vectors: Vec<IndexedVector>) -> Result<Self, DbError> {
        if vectors.is_empty() {
            Err(DbError::IndexInitError)
        } else {
            let dim = vectors[0].vector.len();

            let mut point_map = HashMap::with_capacity(vectors.len());
            for iv in vectors.iter() {
                point_map.insert(iv.id, iv.vector.clone());
            }
            let root_node = Self::build_recursive(&mut vectors, 0, dim);
            Ok(KDTree {
                dim,
                root: Some(root_node),
                point_map,
            })
        }
    }

    // Builds the tree recursively with given vectors and returns the pointer of the root node
    pub fn build_recursive(
        vectors: &mut [IndexedVector],
        depth: usize,
        dim: usize,
    ) -> Box<KDTreeNode> {
        if vectors.is_empty() {
            panic!("Cannot build from an empty slice recursively");
        }

        let axis = depth % dim;
        let mid_idx = vectors.len() / 2;

        vectors.select_nth_unstable_by(mid_idx, |a, b| {
            let a_at_axis = a.vector[axis];
            let b_at_axis = b.vector[axis];
            a_at_axis.partial_cmp(&b_at_axis).unwrap_or(Ordering::Equal)
        });

        // Using swap so that we don't need to clone the whole vector
        let mut median_vec = IndexedVector {
            id: 0,
            vector: vec![],
        }; // dummy
        std::mem::swap(&mut vectors[mid_idx], &mut median_vec);

        let (left_points, right_points_with_median) = vectors.split_at_mut(mid_idx);
        let right_points = &mut right_points_with_median[1..]; // Exclude the swapped-out median

        let left = if left_points.is_empty() {
            None
        } else {
            Some(Self::build_recursive(left_points, depth + 1, dim))
        };

        let right = if right_points.is_empty() {
            None
        } else {
            Some(Self::build_recursive(right_points, depth + 1, dim))
        };

        Box::new(KDTreeNode {
            indexed_vector: median_vec,
            split_dim: axis,
            left,
            right,
            is_deleted: false,
        })
    }

    pub fn insert_point(&mut self, new_vector: IndexedVector) {
        // use a traverse function to get the final leaf where this belongs
        if self.root.is_none() {
            self.root = Some(Box::new(KDTreeNode {
                indexed_vector: new_vector,
                split_dim: 0,
                left: None,
                right: None,
                is_deleted: false,
            }));
            return;
        }

        let mut current_link = &mut self.root;
        let mut depth = 0;
        let dim = self.dim;

        while let Some(ref mut node_box) = current_link {
            let axis = depth % dim;
            let current_node = node_box.as_mut();

            let va = new_vector.vector[axis];
            let vb = current_node.indexed_vector.vector[axis];

            if va <= vb {
                current_link = &mut current_node.left;
            } else {
                current_link = &mut current_node.right;
            }
            depth += 1;
        }

        // Assign the new node to current link which is &mut Option<Box<KDTreeNode>>
        let axis = depth % dim;
        *current_link = Some(Box::new(KDTreeNode {
            indexed_vector: new_vector,
            split_dim: axis,
            left: None,
            right: None,
            is_deleted: false,
        }))
    }

    // Deletes the point by first finding the corresponding node using DFS and then deleting
    // Returns true if point found and deleted, else false
    // First make a lookup of vector from map, then traverse the tree to obtain the point and mark it as deleted
    pub fn delete_point(&mut self, point_id: PointId) -> bool {
        if let Some(vector_to_delete) = self.point_map.get(&point_id) {
            let found_and_deleted = Self::find_and_mark_recursive(
                &mut self.root,
                vector_to_delete,
                point_id,
                0,
                self.dim,
            );

            if found_and_deleted {
                self.point_map.remove(&point_id);
            }

            return found_and_deleted;
        }
        false
    }

    // Recursively finds and marks a node as deleted,
    fn find_and_mark_recursive(
        node_opt: &mut Option<Box<KDTreeNode>>,
        target_vector: &DenseVector,
        target_id: PointId,
        depth: usize,
        dim: usize,
    ) -> bool {
        if let Some(node) = node_opt {
            if node.indexed_vector.id == target_id {
                node.is_deleted = true;
                return true;
            }

            let axis = depth % dim;
            let target_val = target_vector[axis];
            let node_val = node.indexed_vector.vector[axis];

            if target_val < node_val {
                Self::find_and_mark_recursive(
                    &mut node.left,
                    target_vector,
                    target_id,
                    depth + 1,
                    dim,
                )
            } else if target_val > node_val {
                Self::find_and_mark_recursive(
                    &mut node.right,
                    target_vector,
                    target_id,
                    depth + 1,
                    dim,
                )
            } else {
                // Need to check both right and left nodes in this case
                let left_found = Self::find_and_mark_recursive(
                    &mut node.left,
                    target_vector,
                    target_id,
                    depth + 1,
                    dim,
                );
                let right_found = Self::find_and_mark_recursive(
                    &mut node.right,
                    target_vector,
                    target_id,
                    depth + 1,
                    dim,
                );
                left_found || right_found
            }
        } else {
            false
        }
    }

    pub fn search_top_k(
        &self,
        query_vector: DenseVector,
        k: usize,
        dist_type: Similarity,
    ) -> Vec<(PointId, f32)> {
        //Searches for top k closest vectors according to specified metric

        if self.root.is_none() || k == 0 {
            return Vec::new();
        }

        let mut best_neighbours = BinaryHeap::with_capacity(k);

        self.search_recursive(
            &self.root,
            &query_vector,
            k,
            &mut best_neighbours,
            0,
            dist_type,
        );

        best_neighbours
            .into_sorted_vec()
            .iter()
            .map(|neighbor| (neighbor.id, neighbor.distance))
            .collect()
    }

    fn search_recursive(
        &self,
        node_opt: &Option<Box<KDTreeNode>>,
        query_vector: &DenseVector,
        k: usize,
        heap: &mut BinaryHeap<Neighbor>,
        depth: usize,
        dist_type: Similarity,
    ) {
        // Base case is that we hit a leaf node don't do anything
        if let Some(node) = node_opt {
            let axis = depth % self.dim;

            let (near_side, far_side) = if query_vector[axis] <= node.indexed_vector.vector[axis] {
                (&node.left, &node.right)
            } else {
                (&node.right, &node.left)
            };

            // Recurse on near side first
            self.search_recursive(&near_side, query_vector, k, heap, depth + 1, dist_type);

            // Process the current node
            if !node.is_deleted {
                //TODO: Use square distance in distance, why is there overhead of square
                let distance = distance(query_vector, &node.indexed_vector.vector, dist_type);
                if heap.len() < k {
                    heap.push(Neighbor {
                        id: node.indexed_vector.id,
                        distance,
                    });
                } else if distance < heap.peek().unwrap().distance {
                    heap.pop();
                    heap.push(Neighbor {
                        id: node.indexed_vector.id,
                        distance,
                    });
                }
            }

            // Pruning on the farther side to check if there are better candidates
            //TODO: Change this when implementing square distance
            let dist_to_plane = match dist_type {
                Similarity::Euclidean => query_vector[axis] - node.indexed_vector.vector[axis],
                Similarity::Manhattan => 1.0,
                _ => unreachable!(),
            };

            if heap.len() < k || dist_to_plane < heap.peek().unwrap().distance {
                self.search_recursive(far_side, query_vector, k, heap, depth + 1, dist_type);
            }
        }
    }
}

impl VectorIndex for KDTree {
    fn insert(&mut self, vector: IndexedVector) -> Result<(), DbError> {
        self.insert_point(vector);
        Ok(())
    }

    fn delete(&mut self, point_id: PointId) -> Result<bool, DbError> {
        Ok(self.delete_point(point_id))
    }

    fn search(
        &self,
        query_vector: core::DenseVector,
        similarity: Similarity,
        k: usize,
    ) -> Result<Vec<PointId>, DbError> {
        if matches!(similarity, Similarity::Cosine | Similarity::Hamming) {
            panic!("Cosine and hamming are not suitable similariyt metric when using a KDTree")
        }

        Ok(vec![])
    }
}
