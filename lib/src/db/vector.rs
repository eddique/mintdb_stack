use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt::format;

use super::store::Datastore;
use nalgebra::{
    base::{Const, Dyn, Matrix, VecStorage},
    DVector,
};
use serde_json::{json, Value};
pub type Embedding = Matrix<f64, Dyn, Const<1>, VecStorage<f64, Dyn, Const<1>>>;
use crate::math;

#[derive(Debug, Clone)]
struct CosineDistance {
    cosine_distance: f64,
    document: Value,
}

impl PartialEq for CosineDistance {
    fn eq(&self, other: &Self) -> bool {
        self.cosine_distance == other.cosine_distance
    }
}

impl Eq for CosineDistance {}

impl PartialOrd for CosineDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cosine_distance.partial_cmp(&self.cosine_distance)
    }
}

impl Ord for CosineDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}
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
                                let content =
                                    doc.get("content").unwrap_or(&json!("content")).clone();
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
            let a_cosine = a
                .get("cosine_distance")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
            let b_cosine = b
                .get("cosine_distance")
                .and_then(Value::as_f64)
                .unwrap_or(0.0);
            a_cosine.partial_cmp(&b_cosine).unwrap()
        });
        documents.truncate(10);
        documents
    }
    pub async fn query_embeddings(
        &self,
        idx: &str,
        query_vector: &Embedding,
        top_n: usize,
    ) -> Vec<Value> {
        let mut heap = BinaryHeap::new();
        let lk = self.collections.read().await;
        if let Some(collection) = lk.get(idx) {
            for (key, value) in collection {
                if let Some(doc) = value.as_object() {
                    if let Some(embeddings) = doc.get("embedding") {
                        match serde_json::from_value::<Vec<f64>>(embeddings.clone()) {
                            Ok(v) => {
                                let vector = DVector::from_vec(v);
                                let title = doc.get("title").unwrap_or(&json!("default")).clone();
                                let content =
                                    doc.get("content").unwrap_or(&json!("content")).clone();
                                let link = doc.get("link").unwrap_or(&json!("link")).clone();
                                let cosine_distance = math::cosine_distance(query_vector, &vector);
                                let document = json!({
                                    "id": key,
                                    "title": title,
                                    "content": content,
                                    "link": link,
                                    "cosine_distance": cosine_distance,
                                });
                                if heap.len() < top_n {
                                    heap.push(CosineDistance {
                                        cosine_distance,
                                        document,
                                    });
                                } else if let Some(top) = heap.peek() {
                                    if cosine_distance < top.cosine_distance {
                                        heap.pop();
                                        heap.push(CosineDistance {
                                            cosine_distance,
                                            document,
                                        });
                                    }
                                }
                            }
                            Err(e) => {}
                        }
                    }
                }
            }
        }
        drop(lk);
        let mut result = Vec::with_capacity(heap.len());
        while let Some(item) = heap.pop() {
            result.push(item.document);
        }
        result
    }
}
