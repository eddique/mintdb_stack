use axum::{Json, Router};
use axum::http::Response;
use axum::routing::get;
use serde_json::{json, Value};
use crate::wal;

use crate::err::net;
pub fn config() -> Router {
    Router::new()
        .route("/", get(health))
}

async fn health() -> net::Result<Json<Value>> {
    tokio::task::spawn(wal::flush_wal());
    Ok(Json(json!({
        "ok": true,
    })))
}