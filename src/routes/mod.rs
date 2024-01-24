mod dev;
mod sql;
use axum::Router;

pub fn init() -> Router {
    Router::new()
        .nest("/dev", dev::config())
        .nest("/sql", sql::config())
}
