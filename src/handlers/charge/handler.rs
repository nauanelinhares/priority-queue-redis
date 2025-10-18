use crate::handlers::charge::create_charge::create_charge;
use crate::queue::queue::Queue;
use axum::routing::post;
use axum::Router;
use std::sync::Arc;

pub fn charge_handler(queue: Arc<dyn Queue + Send + Sync>) -> Router {
    Router::new()
        .route("/random", post(create_charge))
        .with_state(queue)
}
