use axum::extract::Path;
use axum::{Router, Json, Extension};
use axum::routing::post;
use nalgebra::DVector;
use serde_json::{Value, json};
use crate::models::dev::{InsertRequest, QueryRequest};
use crate::db::DS;
use crate::err::net;

pub fn config() -> Router {
    Router::new()
        .route("/insert", post(insert))
        .route("/query", post(query_vectors))
}

async fn insert(
    Json(req): Json<InsertRequest>
) -> net::Result<Json<Value>> {
    println!("@POST /dev/insert");
    let db = DS.get().unwrap();
    db.insert(&req.idx, req.data).await;
    Ok(Json(json!({
        "ok": true,
        "message": "document inserted"
    })))
}

async fn query_vectors(
    Json(req): Json<QueryRequest>
) -> net::Result<Json<Value>> {
    println!("@POST /dev/query");
    let db = DS.get().unwrap();
use nalgebra::DVector;
    let embedding = DVector::from_vec(req.embedding);
    let documents = db.query_vectors(&req.idx, &embedding).await;
    Ok(Json(json!({
        "ok": true,
        "results": documents,
    })))
}