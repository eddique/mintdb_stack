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
    tracing::info!("@POST /sql - action: {} table: {}", &sql.stmt, &sql.tb);
    match db.exec(&sql).await {
        Ok(res) => {
            let topic = format_topic(&sql);
            tokio::spawn(publish(clients, topic, res.to_string()));
            Ok(Json(json!({
                "ok": true,
                "result": res,
            })))
        }
        Err(e) => {
            tracing::error!("@POST /sql - action: {} table: {} error: {}", &sql.stmt, &sql.tb, e.to_string());
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

fn format_topic(sql: &SQL) -> String {
    let mut topic = sql.tb.to_string();
    if let Some(doc) = &sql.doc {
        topic += ":";
        topic += doc;
    }
    if let Some(key) = &sql.key {
        topic += ":";
        topic += key;
    }
    format!("{}", topic)
}