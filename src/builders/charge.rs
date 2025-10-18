use crate::builders::payment_discrimination::build_payment_discrimination;
use crate::structs::charge::{
    Charge, PaymentDiscriminator, PaymentStatus, PAID, PARTIALLY_PAID, UNPAID,
};
use chrono::Utc;
use rand::seq::IndexedRandom;
use uuid::Uuid;

pub fn build_charge() -> Charge {
    let mut rng = rand::rng();
    let binding = [PAID, PARTIALLY_PAID, UNPAID];
    let payment_status = binding.choose(&mut rng).unwrap().to_string();
    let id = generate_random_uuid();
    let created_at = Utc::now();
    let updated_at = Utc::now();
    let payment_discriminator = build_payment_discrimination();

    Charge {
        id,
        created_at,
        updated_at,
        payment_status,
        payment_discriminator,
    }
}

pub fn generate_random_uuid() -> Uuid {
    Uuid::new_v4()
}

pub struct ChargeBuilder {
    pub charge: Charge,
}

impl ChargeBuilder {
    pub fn new() -> Self {
        Self {
            charge: build_charge(),
        }
    }
    pub fn with_payment_status(mut self, payment_status: String) -> Self {
        self.charge.payment_status = payment_status;
        self
    }
    pub fn with_payment_discriminator(
        mut self,
        payment_discriminator: PaymentDiscriminator,
    ) -> Self {
        self.charge.payment_discriminator = payment_discriminator;
        self
    }
}
