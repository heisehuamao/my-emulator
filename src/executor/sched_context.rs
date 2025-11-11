use std::rc::Rc;
use std::task::Waker;
use crate::executor::sched_wake::SchedWake;

#[derive(Debug)]
pub(crate) struct SchedContext {
    task_id: u64,
    timestamp: u64,
}


impl SchedContext {
    pub(crate) fn new(task_id: u64, timestamp: u64) -> Self {
        Self {
            task_id,
            timestamp,
        }
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