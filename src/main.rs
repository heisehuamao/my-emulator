use std::thread;
use std::time::Duration;
use crate::executor::Executor;

mod executor;

fn main() {
    let mut e = Executor::new();
    println!("Hello, world! exe: {:?}", e);

    e.start_thread();
    e.start_thread();
    e.start_thread();

    thread::sleep(Duration::from_secs(2));
}
