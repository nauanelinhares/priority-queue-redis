mod builders;
mod handlers;
mod queue;
mod structs;
use axum::Router;

use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

use crate::{
    handlers::{charge::charge_handler, health::health_handler},
    queue::redis::RedisQueue,
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let port = env::var("REDIS_PORT").unwrap();
    let host = env::var("REDIS_HOST").unwrap();
    println!("🔌 Port: {}", port);
    println!("🔌 Host: {}", host);

    let queue = Arc::new(RedisQueue::new(port, host));

    let app = Router::new()
        .nest("/health", health_handler())
        .nest("/charge", charge_handler(queue));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
