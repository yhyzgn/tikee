#![forbid(unsafe_code)]

use scheduler_worker_sdk::WorkerConfig;

#[tokio::main]
async fn main() {
    let endpoint = std::env::var("SCHEDULER_WORKER_ENDPOINT")
        .unwrap_or_else(|_| "http://0.0.0.0:9998".to_owned());
    let client_instance_id = std::env::var("SCHEDULER_WORKER_INSTANCE_ID")
        .unwrap_or_else(|_| "rust-demo-instance".to_owned());
    let config = WorkerConfig::local(endpoint, client_instance_id);
    println!(
        "Rust worker demo configured: client_instance_id={}, endpoint={}",
        config.client_instance_id, config.endpoint
    );
    println!("Start scheduler server and replace this dry run with WorkerClient::connect() when needed.");
}
