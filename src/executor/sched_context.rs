use std::rc::Rc;
use crate::executor::sched_wake::SchedWake;

pub(crate) struct SchedContext {
    pub(crate) task_id: u64,
}


impl SchedContext {
    pub(crate) fn new(task_id: u64) -> Self {
        Self {
            task_id,
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