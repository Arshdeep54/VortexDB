use crate::{distance, VectorIndex};
use defs::{DbError, DenseVector, IndexedVector, PointId, Similarity};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    vec,
};
use uuid::Uuid;

pub struct KDTree {
    dim: usize,
    root: Option<Box<KDTreeNode>>,
    // In memory point ids, to check existence before O(n) deletion logic
    point_ids: HashSet<PointId>,
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
    // Build an empty index with no points
    pub fn build_empty(dim: usize) -> Self {
        KDTree {
            dim,
            root: None,
            point_ids: HashSet::new(),
        }
    }

    // Builds the vector index from provided vectors, there should atleast be single vector for dim calculation
    pub fn build(mut vectors: Vec<IndexedVector>) -> Result<Self, DbError> {
        if vectors.is_empty() {
            Err(DbError::IndexInitError)
        } else {
            let dim = vectors[0].vector.len();

            let mut point_ids = HashSet::with_capacity(vectors.len());
            for indexed_vector in vectors.iter() {
                point_ids.insert(indexed_vector.id);
            }

            let root_node = Self::build_recursive(&mut vectors, 0, dim);
            Ok(KDTree {
                dim,
                root: Some(root_node),
                point_ids,
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
            id: Uuid::new_v4(),
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
        // Add to point_ids
        self.point_ids.insert(new_vector.id);

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
        }));
    }

    // Returns true if point found and deleted, else false
    pub fn delete_point(&mut self, point_id: &PointId) -> bool {
        if self.point_ids.contains(point_id) {
            let deleted = Self::find_and_mark_deleted(&mut self.root, *point_id);
            if deleted {
                self.point_ids.remove(point_id);
            }
            return deleted;
        }
        false
    }

    fn find_and_mark_deleted(node_opt: &mut Option<Box<KDTreeNode>>, target_id: PointId) -> bool {
        if let Some(node) = node_opt {
            if node.indexed_vector.id == target_id {
                node.is_deleted = true;
                return true;
            }

            // Search left first then right
            Self::find_and_mark_deleted(&mut node.left, target_id)
                || Self::find_and_mark_deleted(&mut node.right, target_id)
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
            self.search_recursive(near_side, query_vector, k, heap, depth + 1, dist_type);

            // Process the current node
            if !node.is_deleted {
                // TODO: Possible overhead, here heap stores sqrt euclidean distance, we can eliminate that by storing squared distances in case of euclidean
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
            let axis_diff = query_vector[axis] - node.indexed_vector.vector[axis];
            let dist_to_plane = match dist_type {
                Similarity::Euclidean => axis_diff.abs(),
                Similarity::Manhattan => axis_diff.abs(),
                _ => 0.0, // Cosine/Hamming - no effective pruning, always search
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
        Ok(self.delete_point(&point_id))
    }

    fn search(
        &self,
        query_vector: DenseVector,
        similarity: Similarity,
        k: usize,
    ) -> Result<Vec<PointId>, DbError> {
        if matches!(similarity, Similarity::Cosine | Similarity::Hamming) {
            return Err(DbError::UnsupportedSimilarity);
        }

        let results = self.search_top_k(query_vector, k, similarity);
        Ok(results.into_iter().map(|(id, _)| id).collect())
    }
}
