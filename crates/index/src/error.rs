use defs::{Dimension, PointId};
use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum IndexError {
    /// Dimension mismatch when inserting vectors into index
    #[snafu(display("Vector dimension mismatch in index: expected {expected}, got {got}"))]
    DimensionMismatch { expected: Dimension, got: Dimension },

    /// Index not initialized
    #[snafu(display("Index has not been initialized"))]
    NotInitialized,

    /// Unsupported similarity metric
    #[snafu(display("Unsupported similarity metric: {metric}"))]
    UnsupportedSimilarity { metric: String },

    /// Point not found in index
    #[snafu(display("Point {id} not found in index"))]
    PointNotFound { id: PointId },

    /// Point already exists in index
    #[snafu(display("Point {id} already exists in index"))]
    PointAlreadyExists { id: PointId },

    /// HNSW-specific errors
    #[snafu(display("HNSW index error: {message}"))]
    HnswError { message: String },

    /// Invalid index parameter
    #[snafu(display("Invalid index parameter '{parameter}': {reason}"))]
    InvalidParameter { parameter: String, reason: String },

    /// Search failed
    #[snafu(display("Search operation failed: {reason}"))]
    SearchFailed { reason: String },

    /// Index is empty
    #[snafu(display("Cannot perform operation on empty index"))]
    EmptyIndex,

    /// Invalid search limit
    #[snafu(display("Invalid search limit: {limit}"))]
    InvalidSearchLimit { limit: usize },
}

pub type Result<T, E = IndexError> = std::result::Result<T, E>;
