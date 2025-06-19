use std::fmt::Error;
use core::{ DenseVector, PointId, };

pub trait VectorIndex {
    fn insert(&self, vector: DenseVector) -> Result<(), Error>;
    fn delete(&self, point_id: PointId) -> Result<(), Error>;
    fn search(&self, query_vector: DenseVector) -> Result<DenseVector, Error>;
    fn build() -> Result<(), Error>;
}
