use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::executor::block_on;

struct PayloadRequest {
    platform: String,
}

struct PayloadResponse {
    message: String,
    timestamp: u128,
}

fn timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
}

async fn hello_world(request: PayloadRequest) -> PayloadResponse {
    println!("Lets simulate a long running task");
    thread::sleep(Duration::from_secs(5));
    println!("long running task is done");
    return PayloadResponse {
        message: format!("Hello World from Rust to {}!", request.platform),
        timestamp: timestamp(),
    };
}

fn main() {
    let started = timestamp();
    let task = async {
        let response =
            hello_world(PayloadRequest {
                platform: String::from("Rust"),
            }).await;
        println!("{}", response.message);
        let time_spent = response.timestamp - started;
        println!("Duration: {} milliseconds", time_spent);
    };
    block_on(task);
}
