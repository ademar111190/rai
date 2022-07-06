use futures::executor::block_on;

mod functions;
mod payload;

fn main() {
    let started = functions::timestamp();
    let task = async {
        let response =
            functions::hello_world(payload::PayloadRequest {
                platform: String::from("Rust"),
            }).await;
        println!("{}", response.message);
        let time_spent = response.timestamp - started;
        println!("Duration: {} milliseconds", time_spent);
    };
    block_on(task);
}
