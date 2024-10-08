use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::Message;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use axum::Extension;
use tokio::sync::RwLock;
use tokio::sync::mpsc::UnboundedSender;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tokio::signal::unix::{signal, Signal, SignalKind};

use crate::routes;

const URI: &str = "0.0.0.0:3000";

#[derive(Debug, Clone)]
pub (crate) struct Client {
    pub uid: String,
    pub topics: Vec<String>,
    pub sender: Option<UnboundedSender<std::result::Result<Message, axum::Error>>>
}

pub (crate) type Clients = Arc<RwLock<HashMap<String, Client>>>;

async fn shutdown_signal() {
    let mut interrupt = signal(SignalKind::interrupt()).unwrap();
    let mut terminate = signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => println!(  "\x1b[38;5;50m...Shutdown signal received...\x1b[0m"),
        _ = terminate.recv() => println!(  "\x1b[38;5;50m...Terminate signal received...\x1b[0m"),
    }
}

pub async fn init() -> std::result::Result<(), axum::BoxError> {
    let clients = Clients::default();
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any);

    let middleware_stack = ServiceBuilder::new()
        .layer(CookieManagerLayer::new())
        .layer(Extension(clients))
        .layer(cors);

    let app = routes::init().layer(middleware_stack);

    let listener = tokio::net::TcpListener::bind(&URI).await?;
    let srv = axum::serve(listener, app);
    println!("...Started web server on {}...", &URI);
    println!(
        "\x1b[38;5;50m...Started admin console on http://{}/...\x1b[0m",
        &URI
    );
    println!("...Started sql api on http://{}/sql...", &URI);
    println!(
        "\x1b[38;5;50m...Started publish api on http://{}/publish...\x1b[0m",
        &URI
    );
    println!("...Started websocket server server on ws://{}/ws...", &URI);
    println!(
        "\x1b[38;5;50m...Started health check endpoint http://{}/health...\x1b[0m",
        &URI
    );
    let server = srv.with_graceful_shutdown(async {
        shutdown_signal().await;
        
        println!(  "\x1b[38;5;50m...Gracefully shutting down...\x1b[0m");
    });
    server.await?;
    Ok(())
}
