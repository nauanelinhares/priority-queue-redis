mod builders;
mod structs;

use builders::charge::{build_charge, ChargeBuilder};
use serde_json;

use crate::structs::charge::UNPAID;

fn main() {
    let charge = ChargeBuilder::new().with_payment_status(UNPAID);

    match serde_json::to_string_pretty(&charge.charge) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Erro ao serializar para JSON: {}", e),
    }
}
