use crate::StorageEngine;
use core::{DbError, DenseVector, Payload, PointId};

pub struct MemoryStorage {
    // define here how MemoryStorage will be defined
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {}
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl StorageEngine for MemoryStorage {
    fn insert_vector(&self, _id: PointId, _vector: DenseVector) -> Result<(), DbError> {
        Ok(())
    }
    fn insert_payload(&self, _id: PointId, _payload: Payload) -> Result<(), DbError> {
        Ok(())
    }
    fn contains_point(&self, _id: PointId) -> Result<bool, DbError> {
        Ok(true)
    }
    fn delete_point(&self, _id: PointId) -> Result<(), DbError> {
        Ok(())
    }
    fn get_payload(&self, _id: PointId) -> Result<Option<Payload>, DbError> {
        Ok(None)
    }
    fn get_vector(&self, _id: PointId) -> Result<Option<DenseVector>, DbError> {
        Ok(None)
    }
}
