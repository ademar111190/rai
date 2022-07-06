use std::time::{SystemTime, UNIX_EPOCH};

struct PayloadRequest {
    platform: String,
}

struct PayloadResponse {
    message: String,
    timestamp: u64,
}

fn hello_world(request: PayloadRequest) -> PayloadResponse {
    return PayloadResponse {
        message: format!("Hello World from Rust to {}!", request.platform),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs(),
    };
}

fn main() {
    let response = hello_world(PayloadRequest {
        platform: String::from("Rust"),
    });
    println!("{}", response.message);
    println!("Timestamp: {}", response.timestamp);
}
