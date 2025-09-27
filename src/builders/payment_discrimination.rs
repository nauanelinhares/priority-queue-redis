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

pub struct PaymentDiscriminationBuilder {
    pub payment_discrimination: PaymentDiscriminator,
}

impl PaymentDiscriminationBuilder {
    pub fn new() -> Self {
        Self {
            payment_discrimination: build_payment_discrimination(),
        }
    }
    pub fn with_total_amount_paid(mut self, total_amount_paid: i64) -> Self {
        self.payment_discrimination.total_amount_paid = total_amount_paid;
        self
    }
    pub fn with_lost_discounts_paid(mut self, lost_discounts_paid: i64) -> Self {
        self.payment_discrimination.lost_discounts_paid = lost_discounts_paid;
        self
    }
    pub fn with_fine_amount_paid(mut self, fine_amount_paid: i64) -> Self {
        self.payment_discrimination.fine_amount_paid = fine_amount_paid;
        self
    }
    pub fn with_interest_amount_paid(mut self, interest_amount_paid: i64) -> Self {
        self.payment_discrimination.interest_amount_paid = interest_amount_paid;
        self
    }
    pub fn with_additional_paid(mut self, additional_paid: i64) -> Self {
        self.payment_discrimination.additional_paid = additional_paid;
        self
    }
}
