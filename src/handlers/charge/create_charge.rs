use axum::extract::State;
use axum::response::Json;
use serde_json::{json, Value};

use crate::builders::charge::ChargeBuilder;
use crate::queue::queue::Queue;
use std::sync::Arc;

pub async fn create_charge(State(queue): State<Arc<dyn Queue + Send + Sync>>) -> Json<Value> {
    let charge = ChargeBuilder::new();
    queue
        .set(
            format!("charge:{}", charge.charge.id),
            serde_json::to_string(&charge.charge).unwrap(),
        )
        .unwrap();
    Json(json!(charge.charge))
}
