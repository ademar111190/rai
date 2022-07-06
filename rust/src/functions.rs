use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use futures::executor::block_on;
use rifgen::rifgen_attr::generate_interface;

pub struct Rai;

impl Rai {
    #[generate_interface]
    pub fn timestamp() -> u64 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
    }

    #[generate_interface]
    pub fn hello_world_sync(platform: String) -> String {
        let task = async {
            let result = Self::hello_world(platform).await;
            return result;
        };
        return block_on(task);
    }

    pub async fn hello_world(platform: String) -> String {
        println!("Lets simulate a long running task");
        thread::sleep(Duration::from_secs(5));
        println!("long running task is done");
        return format!("Hello World from Rust to {}!", platform);
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::functions::Rai;

    #[test]
    fn timestamp_returns_bigger_than_zero() {
        let result = Rai::timestamp();
        assert!(result > 0);
    }

    #[test]
    fn hello_world_message() {
        let task = async {
            let result = Rai::hello_world(String::from("Test")).await;
            assert_eq!(result, "Hello World from Rust to Test!");
        };
        block_on(task);
    }

    #[test]
    fn hello_world_sync_message() {
        let result = Rai::hello_world_sync(String::from("Test sync"));
        assert_eq!(result, "Hello World from Rust to Test sync!");
    }
}
