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
                                let title = doc.get("title").unwrap_or(&json!("default")).clone();
                                let content = doc.get("content").unwrap_or(&json!("content")).clone();
                                let link = doc.get("link").unwrap_or(&json!("link")).clone();
                                // let cosine_similarity = math::cosine_similarity(query_vector, &vector);
                                let cosine_distance = math::cosine_distance(query_vector, &vector);
                                // let manhattan_distance = math::manhattan_distance(query_vector, &vector);
                                // let minkowski_distance = math::minkowski_distance(query_vector, &vector, 3.0);
                                // let eucleidean_distance = math::euclidean_distance(query_vector, &vector);
                                // let chebychev_distance = math::chebyshev_distance(query_vector, &vector);
                                let document = json!({
                                    "id": key,
                                    "title": title,
                                    "content": content,
                                    "link": link,
                                    // "cosine_similarity": cosine_similarity,
                                    "cosine_distance": cosine_distance,
                                    // "manhattan_distance": manhattan_distance,
                                    // "minkowski_distance": minkowski_distance,
                                    // "eucleidean_distance": eucleidean_distance,
                                    // "manhattan_distance": manhattan_distance,
                                    // "chebychev_distance": chebychev_distance,
                                });
                                documents.push(document);
                            }
                            Err(e) => {}
                        }
                    }
                    
                }
            }
        }
        drop(lk);
        documents.sort_by(|a, b| {
            let a_cosine = a.get("cosine_distance").and_then(Value::as_f64).unwrap_or(0.0);
            let b_cosine = b.get("cosine_distance").and_then(Value::as_f64).unwrap_or(0.0);
            a_cosine.partial_cmp(&b_cosine).unwrap()
        });
        documents.truncate(10);
        documents
    }
}