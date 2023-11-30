use crate::types::{Data, VectorData};

pub fn vectorize(data: Data) -> Data{
    let parts: Vec<&str> = data.payload.split(" ").collect();
    let mut vector_store: Vec<f32> = Vec::new();
    for part in parts{
        vector_store.push(part.chars().map(|c| c as i32 as f32).sum())
    };
    let vec = VectorData{
        vector: vector_store,
        embedding_type: "idk?".to_string(),
    };
    Data { vector: vec, payload: data.payload, data_type: data.data_type }
}   