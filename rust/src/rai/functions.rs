use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::rai::payload::{PayloadRequest, PayloadResponse};

pub fn timestamp() -> u128 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();
}

pub async fn hello_world(request: PayloadRequest) -> PayloadResponse {
    println!("Lets simulate a long running task");
    thread::sleep(Duration::from_secs(5));
    println!("long running task is done");
    return PayloadResponse {
        message: format!("Hello World from Rust to {}!", request.platform),
        timestamp: timestamp(),
    };
}

#[cfg(test)]
mod tests {
    use crate::block_on;
    use crate::rai::functions::{hello_world, timestamp};
    use crate::rai::payload::PayloadRequest;

    #[test]
    fn timestamp_returns_bigger_than_zero() {
        let result = timestamp();
        assert!(result > 0);
    }

    #[test]
    fn hello_world_message() {
        let task = async {
            let result = hello_world(PayloadRequest {
                platform: String::from("Test"),
            }).await;
            assert_eq!(result.message, "Hello World from Rust to Test!");
        };
        block_on(task);
    }
}
