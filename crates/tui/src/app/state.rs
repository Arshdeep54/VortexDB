use core::{DenseVector, Payload};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum AppState {
    #[default]
    Dashboard,
    Database,
    VectorOperations,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModalType {
    CreateDatabase,
    DeleteDatabase,
    ConfirmDeleteDatabase,
    DatabaseList,
    ListVectors,
    VectorDetails,
    Error,
    Success,
    Failure,
    GetVector,
    InsertVector,
    DeleteVector,
    SearchSimilarVectors,
}

#[derive(Debug, Clone)]
pub struct VectorListItem {
    pub id: u64,
    pub vector: DenseVector,
    pub payload: Option<Payload>,
}

impl VectorListItem {
    pub fn dims(&self) -> usize {
        self.vector.len()
    }

    pub fn snippet(&self, max_dims: usize) -> String {
        if self.vector.is_empty() {
            return "[]".to_string();
        }

        let take = self.vector.len().min(max_dims);
        let snippet = self
            .vector
            .iter()
            .take(take)
            .map(|v| format!("{v:.2}"))
            .collect::<Vec<_>>()
            .join(", ");

        if self.vector.len() > take {
            format!("[{snippet}, ...]")
        } else {
            format!("[{snippet}]")
        }
    }

    pub fn payload_summary(&self) -> String {
        self.payload
            .as_ref()
            .map(|payload| format!("{payload:?}"))
            .unwrap_or_else(|| "None".to_string())
    }
}
