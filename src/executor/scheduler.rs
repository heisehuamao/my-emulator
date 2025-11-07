use std::fmt::Debug;
use std::thread;
use std::time::Duration;
use crate::executor::communication::TinyConnection;
use crate::executor::runqueue::RunQueue;
use crate::executor::sched_param::SchedParams;
use crate::executor::taskmng::SchedTaskMng;

pub(crate) struct Scheduler {
    name: String,
    conn: Option<TinyConnection<String>>,
    task_mng: SchedTaskMng,
    task_run_queue: RunQueue,
}

impl Debug for Scheduler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scheduler {{ name: {} }}", self.name)
    }
}

impl Scheduler {
    pub fn new(name: String) -> Self {
        Scheduler { 
            name, 
            conn: None, 
            task_mng: SchedTaskMng::new(), 
            task_run_queue: RunQueue::new() }
    }
    
    pub fn set_conn(&mut self, conn: TinyConnection<String>) {
        self.conn = Some(conn);
    }
    
    pub fn try_recv(&mut self) -> Result<String, ()> {
        match self.conn { 
            Some(ref mut conn) => {
                conn.try_recv()
            }
            None => {
                Err(())
            }
        }
    }
    
    pub fn run(&mut self, param: SchedParams) {
        loop {
            if let Ok(val) = self.try_recv() {
                println!("thread {} recved: {}", param.get_id(), val);
                if val == "q" {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

