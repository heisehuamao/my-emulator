use std::fmt::Debug;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;
use crate::executor::communication::TinyConnection;
use crate::executor::sched_msg::SchedMsg;
use crate::executor::runqueue::RunQueue;
use crate::executor::sched_context::SchedContext;
use crate::executor::sched_param::SchedParams;
use crate::executor::sched_wake::sched_waker_create;
use crate::executor::task::SchedTask;
use crate::executor::taskmng::SchedTaskMng;

pub(crate) struct Scheduler {
    name: String,
    conn: Option<TinyConnection<SchedMsg>>,
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
    
    pub fn set_conn(&mut self, conn: TinyConnection<SchedMsg>) {
        self.conn = Some(conn);
    }
    
    pub fn try_recv(&mut self) -> Result<SchedMsg, ()> {
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
            if let Ok(mut val) = self.try_recv() {
                println!("thread {} recved: {:?}", param.get_id(), val);
                if val.get_cmd() == "q" {
                    break;
                } else if val.get_cmd() == "new_task" {
                    if let Some(task_func) = val.get_task_func() {
                        // create a new wrapper async task, that call this function
                        let new_task = Box::pin(async move {
                            task_func(String::from("test dyn creation")).await;
                        });
                        let new_sched_task = Rc::new(SchedTask::new(String::from("test"), new_task));
                        self.task_run_queue.push_one_task(new_sched_task);
                    }
                }
            }

            // create a context
            let dummie = Rc::new(SchedContext::new(0));
            let sched_waker = sched_waker_create(dummie);
            let mut ctx = Context::from_waker(&sched_waker);

            // schedule all the tasks in the run-queue
            while let Some(task_wrapper) = self.task_run_queue.take_one_task() {
                match task_wrapper.get_task_fut() {
                    Some(task_fut) => {
                        match task_fut.borrow_mut().as_mut().poll(&mut ctx) {
                            Poll::Pending => {
                                println!("task future pending");
                            }
                            Poll::Ready(r) => {
                                println!("task future ready: {:?}", r);
                            }
                        }
                    }
                    None => {}
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    }
}

