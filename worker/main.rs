use redis;
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    dotenv::dotenv().ok();

    let port = env::var("REDIS_PORT").unwrap();
    let host = env::var("REDIS_HOST").unwrap();

    println!("🔄 Worker iniciado - Modo Loop Contínuo");
    println!("📍 Host: {}", host);
    println!("🔌 Port: {}", port);
    println!("⏱️  Verificando a cada 2 segundos...");
    println!("🔄 Pressione Ctrl+C para parar");
    println!("---");

    let client = redis::Client::open(format!("redis://{}:{}", host, port)).unwrap();

    loop {
        let mut conn = match client.get_connection() {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("❌ Erro ao conectar ao Redis: {}", e);
                thread::sleep(Duration::from_secs(5));
                continue;
            }
        };

        // Buscar todas as chaves que começam com "charge:"
        let keys: Vec<String> = match redis::cmd("KEYS").arg("charge:*").query(&mut conn) {
            Ok(keys) => keys,
            Err(e) => {
                eprintln!("❌ Erro ao buscar chaves: {}", e);
                thread::sleep(Duration::from_secs(2));
                continue;
            }
        };

        if keys.is_empty() {
            continue;
        }

        println!("🔍 Encontradas {} charges no Redis:", keys.len());

        for key in keys {
            // Buscar a charge
            match redis::cmd("GET").arg(&key).query::<String>(&mut conn) {
                Ok(charge_json) => {
                    println!("🔑 Processando: {}", key);
                    println!("📄 Dados: {}", charge_json);

                    // Remover a charge após processamento
                    match redis::cmd("DEL").arg(&key).query::<i32>(&mut conn) {
                        Ok(_) => {
                            println!("✅ Charge processada e removida: {}", key);
                        }
                        Err(e) => {
                            eprintln!("❌ Erro ao remover charge {}: {}", key, e);
                        }
                    }
                    println!("---");
                }
                Err(e) => {
                    eprintln!("❌ Erro ao buscar charge {}: {}", key, e);
                }
            }
        }

        // Aguardar antes da próxima verificação
        thread::sleep(Duration::from_secs(2));
    }
}
