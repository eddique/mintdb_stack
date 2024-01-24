use serde_json::{Value, json};
use axum::{Router, Json, routing::post};
use mintdb_stack::SQL;
use crate::err::net;
use crate::db::DS;

pub fn config() -> Router {
    Router::new()
        .route("/", post(query))
}

async fn query(Json(sql): Json<SQL>) -> net::Result<Json<Value>> {
    let db = DS.get().unwrap();
    match db.exec(&sql).await {
        Ok(res) => {
            Ok(Json(json!({
                "ok": true,
                "result": res,
            })))
        }
        Err(e) => {
            Ok(Json(json!({
                "ok": false,
                "error": e.to_string(),
            })))
        }
    }
}