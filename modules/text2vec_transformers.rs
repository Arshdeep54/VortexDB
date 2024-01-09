use reqwest::blocking::{Client, Response};
use std::collections::HashMap;
use std::fmt;
use serde_derive::Serialize;

pub struct VectorizationResult {
    pub text: String,
    pub dimensions: usize,
    pub vector: Option<Vec<f32>>,
}

#[derive(Serialize)]
struct VectorizationRequest {
    input: String,
    pooling_strategy: String,
}

impl fmt::Display for VectorizationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text: {}\nDimensions: {}\nVector: {:?}", self.text, self.dimensions, self.vector)
    }
}

impl fmt::Debug for VectorizationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Text: {}\nDimensions: {}\nVector: {:?}", self.text, self.dimensions, self.vector)
    }
}

pub async fn vectorize(input: &str, pooling_strategy: &str) -> Result<Response, reqwest::Error>{
    let client = Client::new();
    let mut json_data = HashMap::new();
    json_data.insert("text", input);
    json_data.insert("pooling_strategy", pooling_strategy);
    let response = client
        .post("http://localhost:8000/vectors
        ")
        .json(&VectorizationRequest {
            input: input.to_string(),
            pooling_strategy: pooling_strategy.to_string(),
        })
        .send();
    match response{
        Ok(response) => Ok(response),
        Err(e) => panic!("Error: {}", e),
    }
}