use crate::indexer::indexing::{distance, DataHeap, Indexer, KNNType, Node};
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::cmp::Ordering::Less;
use std::collections::BinaryHeap;

#[derive(Serialize, Deserialize)]
pub struct KDTreeInternals {
    pub kd_tree_allow_update: bool,
    pub current_number_of_kd_tree_nodes: usize,
    pub rebuild_threshold: f32,
    pub previous_tree_size: usize,
    pub rebuild_counter: usize,
}

#[derive(Serialize, Deserialize)]
pub struct KDTreeNode {
    pub key: String,
    pub vector: Vec<f32>,
    pub left: Option<Box<KDTreeNode>>,
    pub right: Option<Box<KDTreeNode>>,
    pub dim: usize,
}

impl Node for KDTreeNode {
    fn _left(&self) -> Option<&dyn Node> {
        self.left.as_deref().map(|x| x as &dyn Node)
    }

    fn _right(&self) -> Option<&dyn Node> {
        self.right.as_deref().map(|x| x as &dyn Node)
    }

    fn _key(&self) -> &str {
        &self.key
    }

    fn _vector(&self) -> &Vec<f32> {
        &self.vector
    }

    fn find_nearest_neighbors<'a>(
        &'a self,
        point: Vec<f32>,
        knn_type: KNNType,
        heap: &'a mut BinaryHeap<DataHeap>,
    ) -> (&'a mut BinaryHeap<DataHeap>, usize) {
        self.find_nearest_neighbor_helper(point, 1, knn_type, heap)
    }
}

impl KDTreeNode {
    pub fn new(data: (String, Vec<f32>), dim: usize) -> KDTreeNode {
        KDTreeNode {
            key: data.0,
            vector: data.1,
            left: None,
            right: None,
            dim,
        }
    }

    fn find_nearest_neighbor_helper<'a>(
        &'a self,
        point: Vec<f32>,
        n_visited: usize,
        knn_type: KNNType,
        distances: &'a mut BinaryHeap<DataHeap>,
    ) -> (&'a mut BinaryHeap<DataHeap>, usize) {
        if distances.is_empty() {
            panic!("Empty heap entered!");
        }

        let mut my_n_visited = n_visited;
        let mut my_distances = distances;

        if self.vector[self.dim] < point[self.dim] && self.left.is_some() {
            let (a, b) = self.left.as_ref().unwrap().find_nearest_neighbor_helper(
                point.clone(),
                my_n_visited,
                knn_type,
                my_distances,
            );
            my_distances = a;
            my_n_visited = b;
        }

        // distance along this node's axis
        let axis_dist = distance(point.clone(), self.vector.clone(), knn_type);
        if axis_dist <= my_distances.peek().unwrap().distance {
            // self can only be nearer than worst if axis_dist is less than worst_dist because axis_dist is a lower bound for self_dist
            let self_dist = distance(point.clone(), self.vector.clone(), knn_type.clone());
            if self_dist < my_distances.peek().unwrap().distance {
                my_distances.pop();
                my_distances.push(DataHeap {
                    key: self.key.clone(),
                    distance: self_dist,
                });
            }

            // bookkeeping
            my_n_visited += 1;

            // same reasoning applies for the far side of the split
            if self.vector[self.dim] < point[self.dim] && self.left.is_some() {
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

pub struct KDTree {
    pub _root: Option<Box<KDTreeNode>>,
    pub _internals: KDTreeInternals,
    pub is_debug_run: bool,
    pub dim: usize,
}

impl Indexer for KDTree {
    // Create an empty tree with default values
    fn new() -> KDTree {
        KDTree {
            _root: None,
            _internals: KDTreeInternals {
                kd_tree_allow_update: true,
                current_number_of_kd_tree_nodes: 0,
                rebuild_threshold: 2.0f32,
                previous_tree_size: 0,
                rebuild_counter: 0,
            },
            is_debug_run: true,
            dim: 0,
        }
    }

    // Add a node
    // If the dimension of the tree is zero, then it becomes equal to the input data
    fn add_node(&mut self, data: (String, Vec<f32>), depth: usize) {

        println!("Adding node: {:?}", data);

        if self._root.is_none() {
            self.dim = data.1.len();
            self._root = Some(Box::new(KDTreeNode::new(data, 0)));
            self._internals.current_number_of_kd_tree_nodes += 1;
            return;
        }

        assert_eq!(self.dim, data.1.len());

        if !self._internals.kd_tree_allow_update {
            println!("KDTree is locked for rebuild");
            return;
        }

        if self._internals.previous_tree_size != 0 {
            let current_ratio: f32 = self._internals.current_number_of_kd_tree_nodes as f32
                / self._internals.previous_tree_size as f32;
            if current_ratio > self._internals.rebuild_threshold {
                self._internals.previous_tree_size =
                    self._internals.current_number_of_kd_tree_nodes;
                self.rebuild();
            }
        } else {
            self._internals.previous_tree_size = self._internals.current_number_of_kd_tree_nodes;
        }

        self._internals.current_number_of_kd_tree_nodes += 1;

        let mut current_node = self._root.as_deref_mut().unwrap();
        let mut current_depth = depth;
        loop {
            let current_dimension = current_depth % self.dim;
            if data.1[current_dimension] < current_node.vector[current_dimension] {
                if current_node.left.is_none() {
                    current_node.left = Some(Box::new(KDTreeNode::new(data, current_dimension)));
                    break;
                } else {
                    current_node = current_node.left.as_deref_mut().unwrap();
                    current_depth += 1;
                }
            } else {
                if current_node.right.is_none() {
                    current_node.right = Some(Box::new(KDTreeNode::new(data, current_dimension)));
                    break;
                } else {
                    current_node = current_node.right.as_deref_mut().unwrap();
                    current_depth += 1;
                }
            }
        }
    }

    // delete a node
    fn delete_node(&mut self, data: String) {
        self._internals.kd_tree_allow_update = false;
        let mut points = self.traversal(0);
        let index = points.iter().position(|x| *x.0 == data).unwrap();
        points.remove(index);
        let mut points = Vec::into_boxed_slice(points);
        self._root = Some(Box::new(create_tree_helper(points.as_mut(), 0)));
        self._internals.kd_tree_allow_update = true;
    }

    // print data for debug
    fn print_tree_for_debug(&self) {
        let iterated: Vec<(String, Vec<f32>)> = self.traversal(0);
        for iter in iterated {
            println!("{}", iter.0);
        }
    }

    // TODO: send the vectors to the database, so that we can get the data from there
    // get knn
    fn get_knn(&self, knn_type: KNNType, k_value: usize, vector: Vec<f32>) {
        let mut insert_heap: BinaryHeap<DataHeap> = BinaryHeap::new();
        let k_nodes = self.traversal(k_value);
        for node in &k_nodes {
            insert_heap.push(DataHeap {
                key: node.0.clone(),
                distance: distance(vector.clone(), node.1.clone(), knn_type),
            });
        }
        let root = self._root();
        let binding = root.unwrap();
        let (heap, n_visited) = binding.find_nearest_neighbors(vector, knn_type, &mut insert_heap);

        // Printt the k - nearest neighbors
        println!("Visited {} nodes", n_visited);
        for point in heap.iter() {
            println!("{}", point.key);
        }
        
    }

    fn _root(&self) -> Option<&dyn Node> {
        self._root.as_deref().map(|x| x as &dyn Node)
    }
}

impl KDTree {

    // traversal
    fn traversal(&self, k_value: usize) -> Vec<(String, Vec<f32>)> {
        let mut result: Vec<(String, Vec<f32>)> = Vec::new();
        inorder_traversal_helper(self._root.as_deref(), &mut result, k_value);
        result
    }

    // rebuild tree
    fn rebuild(&mut self) {
        self._internals.kd_tree_allow_update = false;
        self._internals.rebuild_counter += 1;
        if self.is_debug_run {
            println!(
                "Rebuilding tree..., Rebuild counter: {:?}",
                self._internals.rebuild_counter
            );
        }
        let mut points = Vec::into_boxed_slice(self.traversal(0));
        self._root = Some(Box::new(create_tree_helper(points.as_mut(), 0)));
        self._internals.kd_tree_allow_update = true;
    }
}

// Traversal helper function
fn inorder_traversal_helper(
    node: Option<&KDTreeNode>,
    result: &mut Vec<(String, Vec<f32>)>,
    k_value: usize,
) -> Option<bool> {
    if node.is_none() {
        return None;
    }
    if k_value != 0 && k_value <= result.len() {
        return None;
    }
    let current_node = node.unwrap();
    inorder_traversal_helper(current_node.to_owned().left.as_deref(), result, k_value);
    result.push((current_node.key.clone(), current_node.vector.clone()));
    inorder_traversal_helper(current_node.to_owned().right.as_deref(), result, k_value);

    Some(true)
}

// Rebuild tree helper functions
fn create_tree_helper(points: &mut [(String, Vec<f32>)], dim: usize) -> KDTreeNode {
    let points_len = points.len();
    if points_len == 1 {
        return KDTreeNode {
            key: points[0].0.clone(),
            vector: points[0].1.clone(),
            left: None,
            right: None,
            dim,
        };
    }

    // Split around the median
    let pivot = quickselect_by(points, points_len / 2, &|a, b| {
        a.1[dim].partial_cmp(&b.1[dim]).unwrap()
    });

    let left = Some(Box::new(create_tree_helper(
        &mut points[0..points_len / 2],
        (dim + 1) % pivot.1.len(),
    )));
    let right = if points.len() >= 3 {
        Some(Box::new(create_tree_helper(
            &mut points[points_len / 2 + 1..points_len],
            (dim + 1) % pivot.1.len(),
        )))
    } else {
        None
    };

    KDTreeNode {
        key: pivot.0,
        vector: pivot.1,
        left,
        right,
        dim,
    }
}

fn quickselect_by<T>(arr: &mut [T], position: usize, cmp: &dyn Fn(&T, &T) -> Ordering) -> T
where
    T: Clone,
{
    let mut pivot_index = 0;
    // Need to wrap in another closure or we get ownership complaints.
    // Tried using an unboxed closure to get around this but couldn't get it to work.
    pivot_index = partition_by(arr, pivot_index, &|a: &T, b: &T| cmp(a, b));
    let array_len = arr.len();
    match position.cmp(&pivot_index) {
        Ordering::Equal => arr[position].clone(),
        Ordering::Less => quickselect_by(&mut arr[0..pivot_index], position, cmp),
        Ordering::Greater => quickselect_by(
            &mut arr[pivot_index + 1..array_len],
            position - pivot_index - 1,
            cmp,
        ),
    }
}

fn partition_by<T>(arr: &mut [T], pivot_index: usize, cmp: &dyn Fn(&T, &T) -> Ordering) -> usize {
    let array_len = arr.len();
    arr.swap(pivot_index, array_len - 1);
    let mut store_index = 0;
    for i in 0..array_len - 1 {
        if cmp(&arr[i], &arr[array_len - 1]) == Less {
            arr.swap(i, store_index);
            store_index += 1;
        }
    }
    arr.swap(array_len - 1, store_index);
    store_index
}
