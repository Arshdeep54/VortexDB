use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VectorData {
    pub vector: Vec<f32>,
    pub embedding_type: String,
}

#[derive(Clone, Copy, PartialEq, Default, Debug, Serialize, Deserialize)]
pub enum DataType {
    #[default]
    Text,
    Image,
    Audio,
    Blob,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub vector: VectorData,
    pub payload: String,
    pub data_type: DataType,
}
