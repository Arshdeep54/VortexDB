#[derive(Debug)]
pub enum DataType{
    Text,
    Image,
    Audio,
    Blob
}

pub struct Data{
    pub vector: Vec<u32>,
    pub payload: String,
    pub data_type: DataType
}

#[cfg(test)]
#[test]
fn test_text() {
    let data = Data{
        vector: vec![1,2,3],
        payload: String::from("Hello"),
        data_type: DataType::Text
    };
    assert_eq!(data.payload, "Hello");
    assert_eq!(data.vector, vec![1,2,3]);
}
#[test]
fn test_image() {
    let data = Data{
        vector: vec![1,2,3],
        payload: String::from("Hello"),
        data_type: DataType::Image
    };
    assert_eq!(data.payload, "Hello");
    assert_eq!(data.vector, vec![1,2,3]);
}
#[test]
fn test_blob() {
    let data = Data{
        vector: vec![1,2,3],
        payload: String::from("Hello"),
        data_type: DataType::Blob
    };
    assert_eq!(data.payload, "Hello");
    assert_eq!(data.vector, vec![1,2,3]);
}
#[test]
fn test_audio() {
    let data = Data{
        vector: vec![1,2,3],
        payload: String::from("Hello"),
        data_type: DataType::Audio
    };
    assert_eq!(data.payload, "Hello");
    assert_eq!(data.vector, vec![1,2,3])
}