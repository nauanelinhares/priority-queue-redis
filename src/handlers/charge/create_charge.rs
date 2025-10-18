use axum::response::Json;
use serde_json::{json, Value};

use crate::builders::charge::ChargeBuilder;

pub async fn create_charge() -> Json<Value> {
    let charge = ChargeBuilder::new();
    Json(json!(charge.charge))
}
