use core::DbError;

// use core::{DenseVector, Payload, Point, PointId};
// use std::{fmt::Error, path::PathBuf, sync::Arc};

// use index::{IndexType, VectorIndex};
// use storage::{StorageEngine, StorageType};

// pub struct VectorDb {
//     storage: Arc<dyn StorageEngine>,
//     index: Arc<dyn VectorIndex>,
// }

// impl VectorDb {
//     pub fn new(storage: Arc<dyn StorageEngine>, index: Arc<dyn VectorIndex>) -> Self {
//         Self { storage, index }
//     }

//     pub fn insert(&self, vector: DenseVector, payload: Payload) -> Result<PointId, Error> {
//         // Add to storage and index
//         Ok(0)
//     }

//     pub fn delete(&self, id: PointId) -> Result<(), Error> {
//         // Remove from storage
//         // Remove from index
//         Ok(())
//     }

//     pub fn get(&self, id: PointId) -> Result<Option<Point>, Error> {
//         // Search for the Point with given id in storage
//         Ok(None)
//     }

//     pub fn search(&self, query: DenseVector, limit: usize) -> Result<Vec<(PointId, f32)>, Error> {
//         // Use vector index to find similar vectors
//         // Return vector ids with similarity scores
//         Ok(vec![])
//     }
// }

// pub struct DbConfig {
//     pub storage_type: StorageType,
//     pub index_type: IndexType,
//     pub data_path: PathBuf,
//     pub dimension: usize,
// }

pub fn init_api_server() -> Result<(), DbError> {
    // Initialize the storage engine
    // Initialize the vector index
    // Start server
    Ok(())
}
