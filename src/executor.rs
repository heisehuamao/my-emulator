use std::sync::atomic::{AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::executor::communication::TinyConnection;
use crate::executor::scheduler::Scheduler;

mod scheduler;
mod communication;
mod task;

#[derive(Debug)]
pub struct Executor {
    id: AtomicUsize,
    name: String,
    conn: Option<TinyConnection<String>>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            id: AtomicUsize::new(0),
            name: String::from("default"),
            conn: None,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn start_thread(&mut self) -> usize {
        let new_id = self.id.fetch_add(1, Relaxed);
        let (req_tx, req_rx) = flume::bounded::<String>(10);
        let (rsp_tx, rsp_rx) = flume::bounded::<String>(10);
        let exe_end = TinyConnection::new(rsp_rx, req_tx);
        let thread_end = TinyConnection::new(req_rx, rsp_tx);
        let handle = thread::spawn(move || {
            println!("thread spawned {}", new_id);
            let mut sched = Scheduler::new(new_id.to_string());
            println!("sched is {:?}", sched);
            sched.set_conn(thread_end);
            loop {
                if let Ok(val) = sched.try_recv() {
                    println!("thread {} recved: {}", new_id, val);
                }
                thread::sleep(Duration::from_millis(100));
            }
        });
        _ = exe_end.try_send(String::from("from exe"));
        self.conn = Some(exe_end);
        new_id
    }
}