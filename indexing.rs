use crate::db;
use db::{deserialize, Database};

use rocksdb::IteratorMode;

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
