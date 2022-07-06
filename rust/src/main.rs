use futures::executor::block_on;

mod rai;

fn main() {
    let started = rai::functions::timestamp();
    let task = async {
        let response =
            rai::functions::hello_world(rai::payload::PayloadRequest {
                platform: String::from("Rust"),
            }).await;
        println!("{}", response.message);
        let time_spent = response.timestamp - started;
        println!("Duration: {} milliseconds", time_spent);
    };
    block_on(task);
}
