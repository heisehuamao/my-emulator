use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use crate::executor::Executor;
use crate::executor::runtime::Runtime;
use crate::executor::sched_msg::{AsyncTaskFnBox, SchedMsg};
use crate::network::ethernet::{EthKey, MacAddr};
use crate::network::ipv4::IPv4Addr;
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;
use crate::network::stack::NetworkStack;
use crate::network::user_container::UserContainer;

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


    // stack task
    let test_func: AsyncTaskFnBox = Box::new(move |name: String| {
        Box::pin(async move {
            let start = Instant::now();
            let mut pkt1 = NetworkPacket::new();
            let mut pkt2 = NetworkPacket::new();
            (pkt1, _) = cloned_stk.clone().rx(pkt1).await;
            (pkt2, _) = cloned_stk.clone().tx(pkt2).await;
            for i in 1..3 {
                // Self::sleep(Duration::new(1, 0)).await;
                println!("======== Example::async task {} Hello, {}, time: {}", i, name, start.elapsed().as_millis());
                (pkt1, _) = cloned_stk.clone().rx(pkt1).await;
                (pkt2, _) = cloned_stk.clone().tx(pkt2).await;
                Runtime::sleep(Duration::new(1, 0)).await;
            }

            let mac = MacAddr::from_str("00-10-00-00-aa-bb").unwrap();
            let mac_res = cloned_stk.add_mac(&mac);
            if let Ok(_) = mac_res {
                println!("add mac {} ok", mac);
                let ip1 = IPv4Addr::from_str("1.1.1.1").unwrap();
                let ip2 = IPv4Addr::from_str("1.1.1.2").unwrap();
                let ip_res = cloned_stk.add_ipv4(&ip1, Some(&mac));
                if let Ok(_) = ip_res {
                    println!("adding {} ok", ip1)
                } else {
                    println!("adding {} failed", ip1)
                }

                match cloned_stk.add_ipv4(&ip2, Some(&mac)) {
                    Ok(_) => println!("adding {} ok", ip2),
                    Err(_) => println!("adding {} failed", ip2)
                }

                // add udp
                cloned_stk.add_udp_v4(&ip1, 1);
                cloned_stk.add_udp_v4(&ip2, 1);
            } else {
                println!("adding {} failed", mac)
            }
            println!("======@ example end at {}", start.elapsed().as_millis());

            // show all the resources
            println!("====== MAC resources ======");
            cloned_stk.mac_show_all();

            println!("====== IPv4 resources ======");
            cloned_stk.ipv4_show_all();

            println!("====== UDP resources ======");
            cloned_stk.udp_show_all();
            
        })
    });
    let msg = SchedMsg::new(String::from("new_task"), Some(test_func));
    _ = e.try_send(thread_id, msg);

    // app task
    let cloned_stk = stk.clone();
    let app_func: AsyncTaskFnBox = Box::new(move |name: String| {
        Box::pin(async move {
            let mut app_container = UserContainer::new(cloned_stk);
            loop {
                app_container.run();
                Runtime::sleep(Duration::new(1, 0)).await;
            }
        })
    });
    let msg = SchedMsg::new(String::from("new_task"), Some(app_func));
    _ = e.try_send(thread_id, msg);
    
    while running.load(Ordering::SeqCst)  {
        // wait
        thread::sleep(Duration::from_secs(1));
    }

    println!("join all");
    e.exit();
}
