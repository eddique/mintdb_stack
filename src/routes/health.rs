use axum::{Json, Router};
use axum::http::Response;
use axum::routing::get;
use serde_json::{json, Value};

use crate::err::net;
pub fn config() -> Router {
    Router::new()
        .route("/health", get(health))
}

async fn health() -> net::Result<Json<Value>> {
    Ok(Json(json!({
        "ok": true,
    })))
}