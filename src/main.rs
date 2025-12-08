use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use crate::executor::Executor;
use crate::executor::runtime::Runtime;
use crate::executor::sched_msg::{AsyncTaskFnBox, SchedMsg};
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;
use crate::network::stack::NetworkStack;

mod executor;
mod network;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    let mut e = Executor::new();
    println!("Hello, world! exe: {:?}", e);

    ctrlc::set_handler(move || {
        println!("Ctrl+C received!");
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let thread_id = e.start_thread();
    // e.start_thread();
    // e.start_thread();
    
    let stk = Arc::new(NetworkStack::new_eth_stack());
    let cloned_stk = stk.clone();


    let test_func: AsyncTaskFnBox = Box::new(move |name: String| {
        Box::pin(async move {
            let start = Instant::now();
            let mut pkt1 = NetworkPacket::new();
            let mut pkt2 = NetworkPacket::new();
            (pkt1, _) = cloned_stk.clone().rx(pkt1).await;
            (pkt2, _) = cloned_stk.clone().tx(pkt2).await;
            for i in 1..10 {
                // Self::sleep(Duration::new(1, 0)).await;
                println!("======== Example::async task {} Hello, {}, time: {}", i, name, start.elapsed().as_millis());
                (pkt1, _) = cloned_stk.clone().rx(pkt1).await;
                (pkt2, _) = cloned_stk.clone().tx(pkt2).await;
                Runtime::sleep(Duration::new(1, 0)).await;
            }
            println!("======@ example end at {}", start.elapsed().as_millis());
        })
    });
    let msg = SchedMsg::new(String::from("new_task"), Some(test_func));
    _ = e.try_send(thread_id, msg);

    while running.load(Ordering::SeqCst)  {
        // wait
        thread::sleep(Duration::from_secs(1));
    }

    println!("join all");
    e.exit();
}
