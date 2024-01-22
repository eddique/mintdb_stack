#![allow(unused)]

use nalgebra::DVector;

pub(crate) fn cosine_similarity(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
    a.dot(b) / (a.norm() * b.norm())
}
pub(crate) fn euclidean_distance(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
    (a - b).norm()
}
pub(crate) fn manhattan_distance(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
    (a - b).iter().map(|x| x.abs()).sum()
}
pub(crate) fn chebyshev_distance(a: &DVector<f64>, b: &DVector<f64>) -> f64 {
    (a - b).iter().map(|x| x.abs()).fold(f64::MIN, f64::max)
}
// - When `p = 1`, the Minkowski distance equals the Manhattan distance.
// - When `p = 2`, it equals the Euclidean distance.
// - When `p -> ∞`, it approaches the Chebyshev distance.
pub(crate) fn minkowski_distance(a: &DVector<f64>, b: &DVector<f64>, p: f64) -> f64 {
    (a - b).iter().map(|x| x.abs().powf(p)).sum::<f64>().powf(1.0 / p)
}