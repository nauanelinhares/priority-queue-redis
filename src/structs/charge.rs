use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Charge {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payment_status: String,
    pub payment_discriminator: PaymentDiscriminator,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentDiscriminator {
    pub total_amount_paid: i64,
    pub lost_discounts_paid: i64,
    pub fine_amount_paid: i64,
    pub interest_amount_paid: i64,
    pub additional_paid: i64,
}

pub type PaymentStatus = str;

pub const PAID: &str = "PAID";
pub const UNPAID: &str = "UNPAID";
pub const PARTIALLY_PAID: &str = "PARTIALLY_PAID";
