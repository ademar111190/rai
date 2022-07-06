use futures::executor::block_on;

mod functions;
mod payload;

fn main() {
    let started = functions::Rai::timestamp();
    let task = async {
        let response = functions::Rai::hello_world(String::from("Rust")).await;
        println!("{}", response);
        let time_spent = functions::Rai::timestamp() - started;
        println!("Duration: {} milliseconds", time_spent);
    };
    block_on(task);
}
