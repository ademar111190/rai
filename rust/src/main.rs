use protobuf::Message;

use crate::messages::{PayloadRequest, PayloadResponse};

mod functions;
mod messages;

fn main() {
    let request = PayloadRequest {
        platform: String::from("Rust"),
        special_fields: Default::default(),
    }.write_to_bytes().unwrap();
    let bytes: &[u8] = request.as_slice();
    let result = functions::Rai::hello_world(bytes);
    let response = PayloadResponse::parse_from_bytes(result.as_slice()).unwrap();
    println!("{}", response);
}
