use std::collections::VecDeque;
use std::rc::Rc;
use crate::executor::task::SchedTask;

pub(crate) struct RunQueue {
    task_vec: VecDeque<Rc<SchedTask>>,
}

impl RunQueue {
    pub(crate) fn new() -> Self {
        Self { task_vec: VecDeque::new() }
    }

    pub(crate) fn take_one_task(&mut self) -> Option<Rc<SchedTask>> {
        self.task_vec.pop_front()
    }
    
    pub(crate) fn push_one_task(&mut self, st: Rc<SchedTask>) {
        self.task_vec.push_back(st);
    }
}