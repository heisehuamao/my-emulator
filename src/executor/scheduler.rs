use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;
use crate::executor::communication::TinyConnection;
use crate::executor::sched_msg::SchedMsg;
use crate::executor::runqueue::RunQueue;
use crate::executor::sched_context::SchedContext;
use crate::executor::sched_param::SchedParams;
use crate::executor::sched_sleep::SchedSleepRing;
use crate::executor::sched_wake::sched_waker_create;
use crate::executor::sleep_node::SleepRet;
use crate::executor::task::SchedTask;
use crate::executor::taskmng::SchedTaskMng;

pub(crate) struct Scheduler {
    name: String,
    conn: RefCell<Option<TinyConnection<SchedMsg>>>,
    task_mng: SchedTaskMng,
    task_run_queue: RunQueue,
    task_sleep_ring: SchedSleepRing,
    curr_running_task: Option<Rc<SchedTask>>,
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
            conn: RefCell::new(None), 
            task_mng: SchedTaskMng::new(), 
            task_run_queue: RunQueue::new(),
            task_sleep_ring: SchedSleepRing::new(100),
            curr_running_task: None,
        }
    }
    
    pub fn set_conn(&self, conn: TinyConnection<SchedMsg>) {
        self.conn.replace(Some(conn));
    }
    
    pub fn try_recv(&self) -> Result<SchedMsg, ()> {
        let conn_res = self.conn.try_borrow();
        let conn_ref = conn_res.map_err(|_| {
            println!("Scheduler::try_recv(): couldn't borrow conn");
            ()
        })?;
        
        // try to receive data
        match conn_ref.as_ref() {
            Some(conn) => {
                conn.try_recv()
            }
            None => {
                Err(())
            }
        }
    }
    
    pub fn run(&self, param: SchedParams) {
        // create a context
        let dummie = Rc::new(SchedContext::new(0, 0));
        let sched_waker = sched_waker_create(dummie);
        let mut ctx = Context::from_waker(&sched_waker);
        
        loop {
            if let Ok(mut val) = self.try_recv() {
                println!("thread {} recved: {:?}", param.get_id(), val);
                if val.get_cmd() == "q" {
                    break;
                } else if val.get_cmd() == "new_task" {
                    if let Some(task_func) = val.get_task_func() {
                        // create a new wrapper async task, that call this function
                        let new_task = task_func(String::from("xxx"));
                        let new_sched_task = Rc::new(SchedTask::new(self.name.clone() + "-task", new_task));

                        // put it into the hashmap
                        if let Ok(_) =self.task_mng.add_task(new_sched_task.clone()) {
                            println!("new task added, {:?}", new_sched_task);
                            // push it to the run queuee
                            self.task_run_queue.push_one_task(new_sched_task);
                        }
                    }
                }
            }

            // schedule all the tasks in the run-queue
            while let Some(task) = self.task_run_queue.take_one_task() {
                match task.get_task_fut() {
                    Some(task_fut) => {
                        match task_fut.borrow_mut().as_mut().poll(&mut ctx) {
                            Poll::Pending => {
                                println!("task future pending");
                            }
                            Poll::Ready(_) => {
                                println!("task future ready: {:?}", task);
                                match self.task_mng.remove_task(task.get_id()) {
                                    Ok(t) => {
                                        println!("task future removed: {:?}", t);
                                    }
                                    Err(_) => {
                                        println!("task future failed to remove: {:?}", task);
                                    }
                                }
                            }
                        }
                    }
                    None => {}
                }
            }

            thread::sleep(Duration::from_millis(1000));
        }
    }
    
    pub(crate) fn sched_sleep(&self, dur: Duration) -> SleepRet {
        // let curr_running_task = match &self.curr_running_task { 
        //     None => panic!("Scheduler has no running task"),
        //     Some(t) => t.clone(),
        // };
        // SleepRet::new(dur, curr_running_task)
        SleepRet::new(dur)
    }
}

