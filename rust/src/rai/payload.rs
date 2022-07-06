#[derive(Eq, PartialEq, Debug)]
pub struct PayloadRequest {
    pub platform: String,
}

#[derive(Eq, PartialEq, Debug)]
pub struct PayloadResponse {
    pub message: String,
    pub timestamp: u128,
}

#[cfg(test)]
mod tests {
    use crate::rai::payload::{PayloadRequest, PayloadResponse};

    #[test]
    fn payload_request_equals() {
        assert_eq!(
            PayloadRequest {
                platform: String::from("Rust")
            },
            PayloadRequest {
                platform: String::from("Rust")
            }
        );
    }

    #[test]
    fn payload_response_equals() {
        assert_eq!(
            PayloadResponse {
                message: String::from("Rust"),
                timestamp: 1,
            },
            PayloadResponse {
                message: String::from("Rust"),
                timestamp: 1,
            }
        );
    }
}
