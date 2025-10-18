// src/queue/queue.rs
pub trait Queue {
    fn set(&self, key: String, value: String) -> Result<(), String>;
    fn get(&self, key: String) -> Result<Option<String>, String>;
}
