use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::executor::block_on;
use protobuf::Message;
use rifgen::rifgen_attr::generate_interface;

use crate::messages::{PayloadRequest, PayloadResponse};

pub struct Rai;

impl Rai {
    #[generate_interface]
    pub fn hello_world(payload: &[u8]) -> Vec<u8> {
        let task = async {
            let request = PayloadRequest::parse_from_bytes(payload).unwrap();
            let result = _hello_world(request).await;
            return result.write_to_bytes().unwrap();
        };
        return block_on(task);
    }
}

fn timestamp() -> i64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64;
}

async fn _hello_world(payload_request: PayloadRequest) -> PayloadResponse {
    println!("Lets simulate a long running task");
    thread::sleep(Duration::from_secs(5));
    println!("long running task is done");
    return PayloadResponse {
        msg: format!("Hello World from Rust to {}!", payload_request.platform),
        timestamp: timestamp(),
        special_fields: Default::default(),
    };
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use protobuf::Message;

    use crate::functions::{_hello_world, Rai, timestamp};
    use crate::messages::{PayloadRequest, PayloadResponse};

    #[test]
    fn timestamp_returns_bigger_than_zero() {
        let result = timestamp();
        assert!(result > 0);
    }

    #[test]
    fn hello_world_message() {
        let task = async {
            let result = _hello_world(PayloadRequest {
                platform: String::from("Test"),
                special_fields: Default::default(),
            }).await;
            assert_eq!(result.msg, "Hello World from Rust to Test!");
        };
        block_on(task);
    }

    #[test]
    fn hello_world_rai_message() {
        let request = PayloadRequest {
            platform: String::from("Test sync"),
            special_fields: Default::default(),
        }.write_to_bytes().unwrap();
        let bytes = request.as_slice();
        let result = Rai::hello_world(bytes);
        let response = PayloadResponse::parse_from_bytes(result.as_slice()).unwrap();
        assert_eq!(response.msg, "Hello World from Rust to Test sync!");
    }
}
