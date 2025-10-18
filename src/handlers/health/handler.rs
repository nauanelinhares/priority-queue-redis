use crate::handlers::health::healthcheck::health_check;
use axum::{routing::get, Router};

pub fn health_handler() -> Router {
    Router::new().route("/ping", get(health_check))
}
