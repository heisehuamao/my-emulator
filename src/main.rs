use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use crate::executor::Executor;

mod executor;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut e = Executor::new();
    println!("Hello, world! exe: {:?}", e);

    ctrlc::set_handler(move || {
        println!("Ctrl+C received!");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    e.start_thread();
    // e.start_thread();
    // e.start_thread();

    while running.load(Ordering::SeqCst)  {
        // wait
        thread::sleep(Duration::from_secs(1));
    }

    println!("join all");
    e.exit();
}
