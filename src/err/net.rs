use std::convert::Infallible;

use anyhow::Error;
use axum::body::Body;
use axum::response::IntoResponse;
use axum::http::{Response, StatusCode};
pub struct NetError(Error);

pub type Result<T> = std::result::Result<T, NetError>;

impl IntoResponse for NetError {
    fn into_response(self) -> axum::http::Response<Body> {
        let error_message = format!("Something went wrong: {}", self.0);
        let body = axum::body::Bytes::from(error_message.into_bytes());

        axum::http::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(body))
            .unwrap()
    }
}

impl<E> From<E> for NetError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}