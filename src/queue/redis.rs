use crate::queue::queue::Queue;
use redis::{Client, Commands};
use std::sync::Arc;

pub struct RedisQueue {
    client: Arc<Client>, // Wrapper em Arc para permitir Clone
}

impl RedisQueue {
    pub fn new(port: String, host: String) -> Self {
        Self {
            client: Arc::new(Client::open(format!("redis://{}:{}", host, port)).unwrap()),
        }
    }
}

// Adicionar Clone para usar como State
impl Queue for RedisQueue {
    fn set(&self, key: String, value: String) -> Result<(), String> {
        let mut conn = self.client.get_connection().unwrap();
        conn.set(key, value)
            .map_err(|e| format!("Error setting value: {}", e))
    }

    fn get(&self, key: String) -> Result<Option<String>, String> {
        let mut conn = self.client.get_connection().unwrap();
        conn.get(key)
            .map_err(|e| format!("Error getting value: {}", e))
    }
}
