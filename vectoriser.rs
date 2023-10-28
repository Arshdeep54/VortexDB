use crate::types::{Data, DataType};

pub fn vectorize(data: Data) -> Data{
    let parts: Vec<&str> = data.payload.split(" ").collect();
    let mut vector_store: Vec<u32> = Vec::new();
    for part in parts{
        vector_store.push(part.chars().map(|c| c as u32).sum())
    };
    Data { vector: vector_store, payload: data.payload, data_type: data.data_type }
}   