use crate::builders::payment_discrimination::build_payment_discrimination;
use crate::structs::charge::{Charge, PAID, PARTIALLY_PAID, UNPAID};
use chrono::Utc;
use rand::seq::{IndexedRandom, SliceRandom};
use uuid::Uuid;

pub fn build_charge() -> Charge {
    let mut rng = rand::rng();
    let payment_status = [PAID, PARTIALLY_PAID, UNPAID].choose(&mut rng).unwrap();
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
