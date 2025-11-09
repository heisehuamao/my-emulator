use std::fmt::{Debug, Formatter};
use std::sync::atomic::{AtomicUsize};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::executor::communication::TinyConnection;
use crate::executor::sched_msg::{AsyncTaskFnBox, SchedMsg};
use crate::executor::sched_param::SchedParams;
use crate::executor::scheduler::Scheduler;

mod scheduler;
mod communication;
mod task;
mod taskmng;
mod runqueue;
mod sched_param;
mod sched_wake;
mod sched_context;
mod sched_msg;

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
    subs: Vec<SubThread>,
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
            subs: vec![],
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn start_thread(&mut self) -> usize {
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

            let mut sched = Scheduler::new(new_id.to_string());
            println!("sched is {:?}", sched);
            sched.set_conn(thread_end);
            sched.run(params);
        });
        
        
        // self.handles.push(handle);
        // let test_func = Box::new(async |name: String| {
        //     println!("Hello, {name}");
        // });

        // fn make_test_func() -> AsyncTaskFnBox {
        //     Box::new(|name: String| {
        //         Box::pin(async move {
        //             println!("Hello, {name}");
        //         })
        //     })
        // }
        let test_func: AsyncTaskFnBox = Box::new(|name: String| {
            Box::pin(async move {
                println!("Hello, {name}");
            })
        });
        let msg = SchedMsg::new(String::from("new_task"), Some(test_func));
        _ = exe_end.try_send(msg);
        let sun = SubThread::new(exe_end, handle);
        self.subs.push(sun);
        // self.conn = Some(exe_end);
        new_id
    }
    
    fn sub_stop_all(&self) {
        for sub in &self.subs {
            sub.stop();
        }
    }
    
    fn sub_join_all(&mut self) {
        for sub in self.subs.drain(..) {
            sub.join();
        }
    }
    
    pub fn exit(&mut self) {
        self.sub_stop_all();
        self.sub_join_all();
    }
}