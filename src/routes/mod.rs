mod dev;
use axum::Router;

pub fn init() -> Router {
    Router::new()
        .nest("/dev", dev::config())
}
