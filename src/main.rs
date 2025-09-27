mod builders;
mod structs;

use builders::charge::ChargeBuilder;
use serde_json;
use std::env;

use crate::{
    builders::payment_discrimination::PaymentDiscriminationBuilder, structs::charge::UNPAID,
};

fn main() {
    dotenv::dotenv().ok();

    let port = env::var("REDIS_PORT").unwrap();
    let host = env::var("REDIS_HOST").unwrap();

    println!("port: {}", port);
    println!("host: {}", host);

    let client = redis::Client::open(format!("redis://{}:{}", host, port)).unwrap();

    let mut conn = client.get_connection().unwrap();

    loop {
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

        let charge_json = match serde_json::to_string(&charge.charge) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Erro ao serializar para JSON: {}", e);
                return;
            }
        };

        let charge_id = charge.charge.id.to_string();
        let redis_key = format!("charge:{}", charge_id);

        match redis::cmd("SET")
            .arg(&redis_key)
            .arg(&charge_json)
            .query::<String>(&mut conn)
        {
            Ok(_) => {
                println!("✅ Charge salva no Redis com sucesso!");
                println!("🔑 Chave: {}", redis_key);
                println!(
                    "📄 Dados: {}",
                    serde_json::to_string_pretty(&charge.charge).unwrap()
                );
            }
            Err(e) => {
                eprintln!("❌ Erro ao salvar charge no Redis: {}", e);
            }
        }
    }
}
