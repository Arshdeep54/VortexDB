use crate::database::types::Data;
use serde_json;

pub fn serialize(data: Data) -> Vec<u8> {
    serde_json::to_vec(&data).expect("Failed to serialize data")
}

pub fn deserialize(bytes: &[u8]) -> Data {
    serde_json::from_slice(bytes).expect("Failed to deserialize data")
}
