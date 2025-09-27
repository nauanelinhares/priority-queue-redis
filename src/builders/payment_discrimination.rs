use crate::structs::charge::PaymentDiscriminator;
use rand::Rng;

pub fn build_payment_discrimination() -> PaymentDiscriminator {
    let mut rng = rand::rng();

    let total_amount_paid = rng.random_range(1000..=50000);
    let lost_discounts_paid = rng.random_range(0..=5000);
    let fine_amount_paid = rng.random_range(0..=2000);
    let interest_amount_paid = rng.random_range(0..=3000);
    let additional_paid = rng.random_range(0..=1000);

    PaymentDiscriminator {
        total_amount_paid,
        lost_discounts_paid,
        fine_amount_paid,
        interest_amount_paid,
        additional_paid,
    }
}
