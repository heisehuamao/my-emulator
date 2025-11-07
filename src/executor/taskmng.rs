use std::collections::HashMap;
use std::rc::Rc;
use crate::executor::task::SchedTask;

pub(crate) struct SchedTaskMng {
    task_map: HashMap<usize, Rc<SchedTask>>,
}

impl SchedTaskMng {
    pub(crate) fn new() -> Self {
        SchedTaskMng {
            task_map: HashMap::new(),
        }
    }
}