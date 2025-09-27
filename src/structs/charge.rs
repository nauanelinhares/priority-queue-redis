use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Charge {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub payment_status: PaymentStatus,
    pub payment_discriminator: PaymentDiscriminator,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PaymentDiscriminator {
    pub total_amount_paid: i64,
    pub lost_discounts_paid: i64,
    pub fine_amount_paid: i64,
    pub interest_amount_paid: i64,
    pub additional_paid: i64,
}

pub type PaymentStatus = &'static str;

pub const PAID: PaymentStatus = "PAID";
pub const UNPAID: PaymentStatus = "UNPAID";
pub const PARTIALLY_PAID: PaymentStatus = "PARTIALLY_PAID";
