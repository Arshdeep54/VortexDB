use core::{DbError, DenseVector, Payload, PointId};
use std::fmt::Error;
use std::path::PathBuf;
use std::sync::Arc;

pub trait StorageEngine {
    fn insert_vector(&self, id: PointId, vector: DenseVector) -> Result<(), Error>;
    fn get_vector(&self, id: PointId) -> Result<Option<DenseVector>, Error>;
    fn insert_payload(&self, id: PointId, payload: Payload) -> Result<(), DbError>;
    fn get_payload(&self, id: PointId) -> Result<Option<Payload>, DbError>;
    fn delete_point(&self, id: PointId) -> Result<(), DbError>;
    fn contains_point(&self, id: PointId) -> Result<bool, DbError>;
}

pub mod in_memory;
pub mod rocks_db;

pub enum StorageType {
    InMemory,
    RocksDb,
}

pub fn create_storage_engine(
    storage_type: StorageType,
    path: impl Into<PathBuf>,
) -> Result<Arc<dyn StorageEngine>, Error> {
    match storage_type {
        StorageType::InMemory => Ok(Arc::new(in_memory::MemoryStorage::new())),
        StorageType::RocksDb => Ok(Arc::new(rocks_db::RocksDbStorage::new(path))),
    }
}
