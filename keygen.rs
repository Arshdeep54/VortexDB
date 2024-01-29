use super::types::Data;
use sha2::{Digest, Sha256};

pub fn serialize(vector: Data) -> Vec<u8> {
    let bytes = bincode::serialize(&vector).unwrap();
    return bytes;
}

pub fn deserialize(bytes: &[u8]) -> Data {
    let vec = bincode::deserialize(&bytes).unwrap();
    return vec;
}

pub fn hash(vector: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&vector);
    let key = hasher.finalize();
    let key_string = format!("{:x}", key);
    return key_string;
}
