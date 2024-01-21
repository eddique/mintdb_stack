use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct InsertRequest {
    pub idx: String,
    pub data: Value
}

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub idx: String,
    pub embedding: Vec<f64>
}