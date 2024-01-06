use crate::db::{deserialize, Database};
use crate::kd_tree::KDTree;
use crate::types::{Data, VectorData};
use std::ops::Sub;

use rocksdb::IteratorMode;
use serde::de::value;

pub enum KNNType {
    Euclidean,
    Manhattan,
    Hamming,
    Cosine,
}

impl<'a, 'b> Sub<&'b Data> for &'a Data {
    type Output = Data;

    fn sub(self, rhs: &Data) -> Data {
        assert_eq!(self.vector.vector.len(), rhs.vector.vector.len());
        //Checks if the embedding types and data types are the same ( can remove if not required )
        assert_eq!(self.vector.embedding_type, rhs.vector.embedding_type);
        assert_eq!(self.data_type, rhs.data_type);

        Data {
            vector: VectorData {
                vector: self
                    .vector
                    .vector
                    .iter()
                    .zip(rhs.vector.vector.iter())
                    .map(|(&x, &y)| x - y)
                    .collect(),
                embedding_type: self.vector.embedding_type.clone(),
            },
            data_type: self.data_type,
            payload: String::new(),
        }
    }
}

pub fn get_euclidean_knn(database: &Database, input: &Vec<f32>, kvalue: usize) -> Vec<String> {
    let mut all_scores: Vec<(f32, String)> = Vec::<(f32, String)>::new();

    let iter = database.db.iterator(IteratorMode::Start);

    for item in iter {
        let (byte_key, value) = item.unwrap();

        let key = byte_key
            .iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join("");

        let vec = deserialize(&value).vector.vector;
        if vec.len() != input.len() {
            continue;
        }
        let mut score: f32 = 0.0;
        for i in 0..vec.len() {
            score += (input[i] - vec[i]) * (input[i] - vec[i]);
        }

        all_scores.push((score, key));
    }

    all_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut knn = Vec::<String>::new();

    for i in 0..std::cmp::min(kvalue, all_scores.len()) {
        knn.push(all_scores[i].1.clone());
    }
    return knn;
}

pub fn get_manhattan_knn(database: &Database, input: &Vec<f32>, kvalue: usize) -> Vec<String> {
    let mut all_scores: Vec<(f32, String)> = Vec::<(f32, String)>::new();

    let iter = database.db.iterator(IteratorMode::Start);

    for item in iter {
        let (byte_key, value) = item.unwrap();

        let key = byte_key
            .iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join("");

        let vec = deserialize(&value).vector.vector;
        if vec.len() != input.len() {
            continue;
        }
        let mut score: f32 = 0.0;
        for i in 0..vec.len() {
            score += (input[i] - vec[i]).abs();
        }

        all_scores.push((score, key));
    }

    all_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut knn = Vec::<String>::new();

    for i in 0..std::cmp::min(kvalue, all_scores.len()) {
        knn.push(all_scores[i].1.clone());
    }
    return knn;
}

pub fn get_hamming_knn(database: &Database, input: &Vec<f32>, kvalue: usize) -> Vec<String> {
    let mut all_scores: Vec<(f32, String)> = Vec::<(f32, String)>::new();

    let iter = database.db.iterator(IteratorMode::Start);

    for item in iter {
        let (byte_key, value) = item.unwrap();

        let key = byte_key
            .iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join("");

        let vec = deserialize(&value).vector.vector;
        if vec.len() != input.len() {
            continue;
        }
        let mut score: f32 = 0.0;
        for i in 0..vec.len() {
            if input[i] != vec[i] {
                score += 1.0;
            }
        }

        all_scores.push((score, key));
    }

    all_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut knn = Vec::<String>::new();

    for i in 0..std::cmp::min(kvalue, all_scores.len()) {
        knn.push(all_scores[i].1.clone());
    }
    return knn;
}

pub fn get_cosine_knn(database: &Database, input: &Vec<f32>, kvalue: usize) -> Vec<String> {
    let mut all_scores: Vec<(f32, String)> = Vec::<(f32, String)>::new();

    let iter = database.db.iterator(IteratorMode::Start);

    for item in iter {
        let (byte_key, value) = item.unwrap();

        let key = byte_key
            .iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join("");

        let vec = deserialize(&value).vector.vector;
        if vec.len() != input.len() {
            continue;
        }
        let mut a: f32 = 0.0;
        let mut b: f32 = 0.0;
        let mut c: f32 = 0.0;
        for i in 0..vec.len() {
            a += input[i] * vec[i];
            b += input[i] * input[i];
            c += vec[i] * vec[i];
        }
        b = b.sqrt();
        c = c.sqrt();
        all_scores.push((a / (b * c), key));
    }

    all_scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
    all_scores.reverse();

    let mut knn = Vec::<String>::new();

    for i in 0..std::cmp::min(kvalue, all_scores.len()) {
        knn.push(all_scores[i].1.clone());
    }
    return knn;
}

pub fn get_knn(
    database: &Database,
    input: &Vec<f32>,
    kvalue: usize,
    knn_type: KNNType,
) -> Vec<String> {
    //add the newly created tree to the database

    let mut all_scores: Vec<(f32, String)> = Vec::<(f32, String)>::new();

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

    //temporary
    let ret_vec: Vec<String> = Vec::new();
    return ret_vec;
}

//create a common function for all types of knn with just different methods of calculation
//check everything works
//work on debug print
//work on remaining functions
//integrate them all
