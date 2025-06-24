// Rewrite needed

use crate::StorageEngine;
use core::{DbError, DenseVector, Payload, PointId};
use std::path::PathBuf;

//TODO: Implement RocksDbStorage with necessary fields and implementations
pub struct RocksDbStorage {
    pub path: PathBuf,
}

impl RocksDbStorage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        RocksDbStorage { path: path.into() }
    }
}

impl StorageEngine for RocksDbStorage {
    fn insert_vector(&self, _id: PointId, _vector: DenseVector) -> Result<(), std::fmt::Error> {
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
    fn get_vector(&self, _id: PointId) -> Result<Option<DenseVector>, std::fmt::Error> {
        Ok(None)
    }
}
