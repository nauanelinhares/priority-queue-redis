use crate::handlers::charge::create_charge::create_charge;
use axum::routing::post;
use axum::Router;

pub fn charge_handler() -> Router {
    Router::new().route("/random", post(create_charge))
}
