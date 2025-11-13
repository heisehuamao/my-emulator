use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use crate::executor::communication::TinyConnection;
use crate::executor::sched_msg::{AsyncTaskFnBox, SchedMsg};
use crate::executor::sched_param::SchedParams;
use crate::executor::scheduler::Scheduler;
use crate::executor::sleep_async_node::SleepAsyncNode;
use crate::executor::runtime::Runtime;

mod scheduler;
mod communication;
mod task;
mod taskmng;
mod runqueue;
mod sched_param;
mod sched_wake;
mod sched_context;
mod sched_msg;
mod sched_sleep_ring;
mod sleep_async_node;
mod runtime;

struct SubThread {
    conn: TinyConnection<SchedMsg>,
    handle: thread::JoinHandle<()>,
}

impl SubThread {
    fn new(conn: TinyConnection<SchedMsg>,
           handle: thread::JoinHandle<()>) -> Self {
        Self { conn, handle }
    }

    fn stop(&self) {
        // send the info first
        loop {
            match self.conn.try_send(SchedMsg::new(String::from("q"), None)) {
                Ok(_) => break,
                Err(_) => thread::sleep(Duration::from_millis(100)),
            }
        }
    }
    
    fn join(self) {
        match self.handle.join() {
            Ok(_) => {},
            Err(e) => {}
        }
    }
}

pub struct Executor {
    id: AtomicUsize,
    name: String,
    subs: RefCell<Vec<SubThread>>,
    // conn: Option<TinyConnection<String>>,
    // handles: Vec<thread::JoinHandle<()>>,
}

impl Debug for Executor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            id: AtomicUsize::new(0),
            name: String::from("default"),
            subs: RefCell::new(vec![]),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn start_thread(&self) -> usize {
        let new_id = self.id.fetch_add(1, Relaxed);

        // create communication tunnel
        let (req_tx, req_rx) = flume::bounded::<SchedMsg>(10);
        let (rsp_tx, rsp_rx) = flume::bounded::<SchedMsg>(10);
        let exe_end = TinyConnection::new(rsp_rx, req_tx);
        let thread_end = TinyConnection::new(req_rx, rsp_tx);

        // create a new thread
        let handle = thread::spawn(move || {
            println!("thread spawned {}", new_id);
            let params = SchedParams::new(new_id, String::from("tmp"));

            let sched = Rc::new(Scheduler::new(new_id.to_string()));
            println!("sched is {:?}", sched);
            sched.set_conn(thread_end);

            // set up the sched environment and start running
            Runtime::set_scheduler(&sched);
            sched.run(params);
            Runtime::clear_scheduler();
        });
        
        let test_func: AsyncTaskFnBox = Box::new(|name: String| {
            Box::pin(async move {
                // Self::sleep(Duration::new(1, 0)).await;
                println!("Hello, {}", name);
                Runtime::sleep(Duration::new(1, 0)).await;
            })
        });
        let msg = SchedMsg::new(String::from("new_task"), Some(test_func));
        _ = exe_end.try_send(msg);
        let sun = SubThread::new(exe_end, handle);
        self.subs.borrow_mut().push(sun);
        // self.conn = Some(exe_end);
        new_id
    }
    
    fn sub_stop_all(&self) {
        let borrow_subs = self.subs.borrow();
        let subs = borrow_subs.iter();
        for sub in subs {
            sub.stop();
        }
    }
    
    fn sub_join_all(&self) {
        let mut borrow_subs = self.subs.borrow_mut();
        for sub in borrow_subs.drain(..) {
            sub.join();
        }
    }
    
    pub fn exit(&self) {
        self.sub_stop_all();
        self.sub_join_all();
    }

    // pub fn sleep(dur: Duration) -> SleepRet {
    //     let res = get_scheduler();
    //     match res {
    //         Some(sched) => {
    //             sched.sched_sleep(dur)
    //         }
    //         None => {
    //             panic!("Scheduler not running");
    //         }
    //     }
    // }
}