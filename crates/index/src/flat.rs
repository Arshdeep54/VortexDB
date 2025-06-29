use core::{DbError, DenseVector, IndexedVector, PointId, Similarity};

use crate::{distance, VectorIndex};

pub struct FlatIndex {
    index: Vec<IndexedVector>,
}

impl FlatIndex {
    pub fn new() -> Self {
        Self { index: Vec::new() }
    }

    pub fn build(vectors: Vec<IndexedVector>) -> Self {
        FlatIndex { index: vectors }
    }
}

impl Default for FlatIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl VectorIndex for FlatIndex {
    fn insert(&mut self, vector: IndexedVector) -> Result<(), DbError> {
        self.index.push(vector);
        Ok(())
    }

    fn delete(&mut self, point_id: PointId) -> Result<bool, DbError> {
        if let Some(pos) = self.index.iter().position(|vector| vector.id == point_id) {
            self.index.remove(pos);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn search(
        &self,
        query_vector: DenseVector,
        similarity: Similarity,
        k: usize,
    ) -> Result<Vec<PointId>, DbError> {
        let mut scores = self
            .index
            .iter()
            .map(|point| {
                (
                    point.id,
                    distance(point.vector.clone(), query_vector.clone(), similarity),
                )
            })
            .collect::<Vec<_>>();

        // Sorting logic according to type of metric used
        match similarity {
            Similarity::Euclidean | Similarity::Manhattan | Similarity::Hamming => {
                scores.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            }
            Similarity::Cosine => {
                scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            }
        }

        Ok(scores
            .into_iter()
            .take(k)
            .map(|(id, _)| id)
            .collect::<Vec<_>>())
    }
}
