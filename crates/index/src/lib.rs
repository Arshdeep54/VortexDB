use core::{DenseVector, PointId};
use std::fmt::Error;

pub trait VectorIndex {
    fn insert(&self, vector: DenseVector) -> Result<(), Error>;
    fn delete(&self, point_id: PointId) -> Result<(), Error>;
    fn search(&self, query_vector: DenseVector) -> Result<DenseVector, Error>;
    // fn build() -> Result<(), Error>; move this to impl for dyn compatibility
}

pub enum IndexType {
    Flat,
    KDTree,
    HNSW,
}
