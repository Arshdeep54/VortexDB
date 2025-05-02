use crate::database::types::Data;

pub fn serialize(vector: Data) -> Vec<u8> {
    // TODO: Implement proper serialization
    // Returns empty bytes for now
    Vec::new()
}

pub fn deserialize(bytes: &[u8]) -> Data {
    // TODO: Implement proper deserialization
    // Returns default/empty data for now
    Data::default()
}

