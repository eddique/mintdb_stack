use axum::Extension;
use axum::extract::ws::Message;
use serde_json::{Value, json};
use axum::{Router, Json, routing::post};
use mintdb_stack::SQL;
use crate::err::net;
use crate::db::DS;
use crate::net::Clients;

pub fn config() -> Router {
    Router::new()
        .route("/", post(query))
}

async fn query(
    Extension(clients): Extension<Clients>,
    Json(sql): Json<SQL>,
) -> net::Result<Json<Value>> {
    let db = DS.get().unwrap();
    match db.exec(&sql).await {
        Ok(res) => {
            tokio::spawn(publish(clients, format!("users:1:name"), res.to_string()));
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

async fn publish(clients: Clients, topic: String, message: String) {
    clients
        .read()
        .await
        .iter()
        .filter(|(_, client)| client.topics.iter().any(|t| topic.starts_with(t)))
        .for_each(|(_, client)| {
            if let Some(sender) = &client.sender {
                let _ = sender.send(Ok(Message::Text(message.clone())));
            }
        });
}