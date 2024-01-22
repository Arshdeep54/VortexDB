#[cfg(test)]
mod tests {
    use kd_tree::KDTree;
    use types::{Data, DataType, VectorData};

    use crate::{kd_tree, types};

    #[test]
    fn test_text() {
        let vector = VectorData {
            vector: vec![1.0, 2.0, 3.0],
            embedding_type: String::from("text"),
        };
        let data = Data {
            vector: vector,
            payload: String::from("Hello"),
            data_type: DataType::Text,
        };
        assert_eq!(data.payload, "Hello");
    }
    #[test]
    fn test_image() {
        let vector = VectorData {
            vector: vec![1.0, 2.0, 3.0],
            embedding_type: String::from("image"),
        };
        let data = Data {
            vector: vector,
            payload: String::from("Hello"),
            data_type: DataType::Image,
        };
        assert_eq!(data.payload, "Hello");
    }
    #[test]
    fn test_blob() {
        let vector = VectorData {
            vector: vec![1.0, 2.0, 3.0],
            embedding_type: String::from("binary"),
        };
        let data = Data {
            vector: vector,
            payload: String::from("Hello"),
            data_type: DataType::Blob,
        };
        assert_eq!(data.payload, "Hello");
    }
    #[test]
    fn test_audio() {
        let vector = VectorData {
            vector: vec![1.0, 2.0, 3.0],
            embedding_type: String::from("audio"),
        };
        let data = Data {
            vector: vector,
            payload: String::from("Hello"),
            data_type: DataType::Audio,
        };
        assert_eq!(data.payload, "Hello");
    }

    fn random_data(a: f32, b: f32, c: f32) -> Data {
        let vector = VectorData {
            vector: vec![a, b, c],
            embedding_type: String::from("text"),
        };
        let data = Data {
            vector: vector,
            payload: String::from("Hello"),
            data_type: DataType::Text,
        };
        return data;
    }

    #[test]
    fn create_tree() {
        let mut tree = KDTree::new();
        tree.dim = 3;
        let data = random_data(1.0, 2.0, 3.0);
        tree.add_node(data, 0);
        let data = random_data(0.0, -3.0, 7.0);
        tree.add_node(data, 0);
        let data = random_data(5.0, -9.0, 3.0);
        tree.add_node(data, 0);
    }

    #[test]
    #[should_panic]
    fn test_input_incorrect_vector() {
        let mut tree = KDTree::new();
        tree.dim = 2;
        let data = random_data(1.0, 2.0, 3.0);
        tree.add_node(data, 0);
    }

    #[test]
    fn rebuild_check() {}
}
