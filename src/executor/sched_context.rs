use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::task::Waker;
use crate::executor::sched_wake::SchedWake;
use crate::executor::scheduler::Scheduler;
use crate::executor::task::SchedTask;

#[derive(Debug)]
pub(crate) struct SchedContext {
    task_id: u64,
    curr_time_usec: u64,
    curr_scheduler: Rc<Scheduler>,
    curr_task: RefCell<Option<Rc<SchedTask>>>,
}


impl SchedContext {
    pub(crate) fn new(task_id: u64, timestamp: u64, sched: Rc<Scheduler>) -> Self {
        Self {
            task_id,
            curr_time_usec: timestamp,
            curr_scheduler: sched,
            curr_task: RefCell::new(None),
        }
    }

    pub(crate) fn get_time_usec(&self) -> u64 {
        self.curr_time_usec
    }
    
    pub(crate) fn get_curr_scheduler(&self) -> &Scheduler {
        &self.curr_scheduler
    }
    
    pub(crate) fn get_curr_task(&self) -> Option<Rc<SchedTask>> {
        match self.curr_task.borrow().as_ref() {
            None => None,
            Some(t) => {
                Some(t.clone())
            }
        }
    }
    
    pub(crate) fn set_curr_task(&self, task: Option<Rc<SchedTask>>) {
        self.curr_task.replace(task);
    }
}

impl SchedWake for SchedContext {
    fn wake(self: Rc<SchedContext>) {
        println!("--------wake----------");
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Rc<SchedContext>) {
        println!("--------wake_by_ref----------");
    }
}

pub(crate) fn get_sched_context_from_waker(waker: &Waker) -> Rc<SchedContext>{
    unsafe {
        let ptr = waker.data();
        let rc_data = Rc::from_raw(ptr as *const SchedContext);
        let cloned = rc_data.clone();
        std::mem::forget(rc_data);
        cloned
    }
}