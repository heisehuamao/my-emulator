use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use crate::executor::task::SchedTask;

pub(crate) struct RunQueue {
    task_vec: RefCell<VecDeque<Rc<SchedTask>>>,
}

impl RunQueue {
    pub(crate) fn new() -> Self {
        Self { task_vec: RefCell::new(VecDeque::new()) }
    }

    pub(crate) fn take_one_task(&self) -> Option<Rc<SchedTask>> {
        self.task_vec
            .borrow_mut()
            .pop_front()
    }
    
    pub(crate) fn push_one_task(&self, st: Rc<SchedTask>) {
        self.task_vec
            .borrow_mut()
            .push_back(st);
    }
}