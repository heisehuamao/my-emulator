use std::rc::Rc;
use crate::executor::task::SchedTask;

pub(crate) struct RunQueue {
    task_vec: Vec<Rc<SchedTask>>,
}

impl RunQueue {
    pub(crate) fn new() -> Self {
        Self { task_vec: Vec::new() }
    }
}