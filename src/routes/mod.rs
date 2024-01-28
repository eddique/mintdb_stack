mod dev;
mod health;
mod sql;
mod ws;
use axum::Router;

pub fn init() -> Router {
    Router::new()
        .nest("/dev", dev::config())
        .nest("/sql", sql::config())
        .nest("/ws", ws::config())
        .nest("/health", health::config())
}
