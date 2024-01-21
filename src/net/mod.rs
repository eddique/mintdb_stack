use std::sync::Arc;

use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use axum::Extension;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::routes;

const URI: &str = "0.0.0.0:3000";

pub async fn init() -> std::result::Result<(), axum::BoxError> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_origin(Any);

    let middleware_stack = ServiceBuilder::new()
        .layer(CookieManagerLayer::new())
        .layer(cors);

    let app = routes::init().layer(middleware_stack);

    tracing::info!("server listening on {URI}");
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
    srv.await?;
    Ok(())
}
