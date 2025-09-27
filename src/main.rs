mod builders;
mod structs;

use builders::charge::ChargeBuilder;
use serde_json;

use crate::{
    builders::payment_discrimination::PaymentDiscriminationBuilder, structs::charge::UNPAID,
};

fn main() {
    let payment_discrimination = PaymentDiscriminationBuilder::new()
        .with_total_amount_paid(0)
        .with_lost_discounts_paid(0)
        .with_fine_amount_paid(0)
        .with_interest_amount_paid(0)
        .with_additional_paid(0)
        .payment_discrimination;

    let charge = ChargeBuilder::new()
        .with_payment_status(UNPAID)
        .with_payment_discriminator(payment_discrimination);

    match serde_json::to_string_pretty(&charge.charge) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Erro ao serializar para JSON: {}", e),
    }
}
