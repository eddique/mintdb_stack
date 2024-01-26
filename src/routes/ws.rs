use std::net::SocketAddr;

use axum::{Router, Extension, Json};
use axum::extract::{WebSocketUpgrade, ConnectInfo, Path};
use axum::extract::ws::{WebSocket, Message};
use axum::response::IntoResponse;
use axum::routing::{get, post, delete};
use axum_extra::TypedHeader;
use axum_extra::headers::UserAgent;
use futures::FutureExt;
use futures_util::stream::StreamExt;
use serde_json::{json, Value};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::err::net;
use crate::net::{Clients, Client};

pub fn config() -> Router {
    Router::new()
        .route("/", post(register_client))
        .route("/:id", delete(unregister))
        .route("/:id", get(websocket))

}

async fn websocket(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    Path(id): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Extension(clients): Extension<Clients>
) -> impl IntoResponse {
    let client = clients.read().await.get(&id).cloned();
    if let Some(c) = client {
        ws.on_upgrade(move |socket| client_connection(socket, addr, id, c, clients));
    }
}
async fn unregister(
    Path(id): Path<String>,
    Extension(clients): Extension<Clients>
) -> net::Result<Json<Value>> {
    match clients.write().await.remove_entry(&id) {
        Some(entry) => {
            Ok(Json(json!({
                "ok": true, 
                "message": "client unregistered", 
                "id": id
            })))
        }
        None => {
            Ok(Json(json!({
                "ok": true, 
                "message": "client id not found", 
                "id": id
            })))
        }
    }
}
async fn register_client(
    Extension(clients): Extension<Clients>
) -> net::Result<Json<Value>>{
    let uid = mintdb_stack::uuid_v4!();
    let url = format!("http://localhost:3000/ws/{}", uid);
    clients.write().await.insert(
        format!("{uid}"),
        Client {
            uid,
            topics: vec![],
            sender: None,
        },
    );
    Ok(Json(json!({
        "ok": true,
        "url": url,
    })))
}

async fn client_connection(mut socket: WebSocket, addr: SocketAddr, id: String, mut client: Client, clients: Clients) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (c_tx, c_rx) = mpsc::unbounded_channel();

    let c_rx = UnboundedReceiverStream::new(c_rx);
    tokio::task::spawn(c_rx.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            println!("Error sending websocket message: {}", e);
        }
    }));

    client.sender = Some(c_tx);

    clients.write().await.insert(format!("{id}"), client);

    tracing::info!("{} connected", id);

    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                tracing::error!("{e}");
                break;
            }
        };
        client_msg(&id, msg, &clients).await;
    }

    clients.write().await.remove(&id);
    tracing::info!("{} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, clients: &Clients) {
    println!("received message from {}: {:?}", id, msg);
    let message = match msg.to_text() {
        Ok(val) => val,
        Err(e) => {
            tracing::error!("{e}");
            return;
        }
    };

    if message == "ping" || message == "ping\n" {
        return;
    }

    // TODO: Handle topics request
    
}