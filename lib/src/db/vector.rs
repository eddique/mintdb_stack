use std::fmt::format;

use super::store::Datastore;
use nalgebra::{base::{Matrix, Dyn, Const, VecStorage}, DVector};
use serde_json::{json, Value};
pub type Embedding = Matrix<f64, Dyn, Const<1>, VecStorage<f64, Dyn, Const<1>>>;
use crate::math;
impl Datastore {
    pub async fn query_vectors(&self, idx: &str, query_vector: &Embedding) -> Vec<Value> {
        let mut documents = vec![];
        let lk = self.collections.read().await;
        if let Some(collection) = lk.get(idx) {
            for (key, value) in collection {
                if let Some(doc) = value.as_object() {
                    if let Some(embeddings) = doc.get("embedding") {
                        match serde_json::from_value::<Vec<f64>>(embeddings.clone()) {
                            Ok(v) => {
                                let vector = DVector::from_vec(v);
                                let title = doc.get("title").unwrap_or(&json!("default")).to_string();
                                let cosine = math::cosine_similarity(&query_vector, &vector);
                                let manhattan_distance = math::manhattan_distance(&query_vector, &vector);
                                let document = json!({
                                    "title": title,
                                    "cosine": cosine,
                                    "manhattan_distance": manhattan_distance,
                                });
                                documents.push(document);
                                println!(
                                    "Document: {key}\n\
                                    cosine_similarity: {}\n\
                                    euclidean_distance: {}\n\
                                    manhattan_distance: {}\n\
                                    chebyshev_distance: {}\n\
                                    minkowski_distance {}\n\
                                    hamming_distance: {}\n",
                                    math::cosine_similarity(&query_vector, &vector),
                                    math::euclidean_distance(&query_vector, &vector),
                                    math::manhattan_distance(&query_vector, &vector),
                                    math::chebyshev_distance(&query_vector, &vector),
                                    math::minkowski_distance(&query_vector, &vector, 3.0),
                                    math::hamming_distance(&query_vector, &vector),
                                );
                            }
                            Err(e) => {}
                        }
                    }
                    
                }
            }
        }
        drop(lk);
        documents.sort_by(|a, b| {
            let a_cosine = a.get("cosine").and_then(Value::as_f64).unwrap_or(0.0);
            let b_cosine = b.get("cosine").and_then(Value::as_f64).unwrap_or(0.0);
            b_cosine.partial_cmp(&a_cosine).unwrap()
        });
        documents.truncate(3);
        documents
    }
}