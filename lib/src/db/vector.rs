use super::store::Datastore;
use nalgebra::{base::{Matrix, Dyn, Const, VecStorage}, DVector};
pub type Embedding = Matrix<f64, Dyn, Const<1>, VecStorage<f64, Dyn, Const<1>>>;
use crate::math;
impl Datastore {
    pub async fn query_vectors(&self, idx: &str, query_vector: &Embedding) {
        let lk = self.collections.read().await;
        if let Some(collection) = lk.get(idx) {
            for (key, value) in collection {
                if let Some(doc) = value.as_object() {
                    if let Some(embeddings) = doc.get("embeddings") {
                        match serde_json::from_value::<Vec<f64>>(embeddings.clone()) {
                            Ok(v) => {
                                let vector = DVector::from_vec(v);
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
    }
}