#[derive(Debug)]
pub struct VectorData {
    pub vector: Vec<f32>,
    pub embedding_type: String
}

#[derive(Debug)]
pub enum DataType{
    Text,
    Image,
    Audio,
    Blob
}

pub struct Data{
    pub vector: VectorData,
    pub payload: String,
    pub data_type: DataType
}

#[cfg(test)]
#[test]
fn test_text() {
    let vector = VectorData{
        vector: vec![1.0,2.0,3.0],
        embedding_type: String::from("text")
    };
    let data = Data{
        vector: vector,
        payload: String::from("Hello"),
        data_type: DataType::Text
    };
    assert_eq!(data.payload, "Hello");
}
#[test]
fn test_image() {
    let vector = VectorData{
        vector: vec![1.0,2.0,3.0],
        embedding_type: String::from("image")
    };
    let data = Data{
        vector: vector,
        payload: String::from("Hello"),
        data_type: DataType::Image
    };
    assert_eq!(data.payload, "Hello");
}
#[test]
fn test_blob() {
    let vector = VectorData{
        vector: vec![1.0,2.0,3.0],
        embedding_type: String::from("binary")
    };
    let data = Data{
        vector: vector,
        payload: String::from("Hello"),
        data_type: DataType::Blob
    };
    assert_eq!(data.payload, "Hello");
}
#[test]
fn test_audio() {
    let vector = VectorData{
        vector: vec![1.0,2.0,3.0],
        embedding_type: String::from("audio")
    };
    let data = Data{
        vector: vector,
        payload: String::from("Hello"),
        data_type: DataType::Audio
    };
    assert_eq!(data.payload, "Hello");
}