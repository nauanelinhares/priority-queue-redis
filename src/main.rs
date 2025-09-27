mod builders;
mod structs;

use builders::charge::build_charge;
use serde_json;

fn main() {
    let charge = build_charge();
    
    match serde_json::to_string_pretty(&charge) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Erro ao serializar para JSON: {}", e),
    }
}
