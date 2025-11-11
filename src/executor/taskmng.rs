use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::executor::task::SchedTask;

pub(crate) struct SchedTaskMng {
    id_generator: Cell<usize>,
    task_map: RefCell<HashMap<usize, Rc<SchedTask>>>,
}

impl SchedTaskMng {
    pub(crate) fn new() -> Self {
        SchedTaskMng {
            id_generator: Cell::new(1),
            task_map: RefCell::new(HashMap::new()),
        }
    }

    pub(crate) fn add_task(&self, t: Rc<SchedTask>) -> Result<(), ()> {
        let mut map = self.task_map.borrow_mut();
        loop {
            let task_id = self.id_generator.get();
            self.id_generator.set(task_id + 1);
            t.set_id(task_id);

            if map.contains_key(&task_id) {
                continue;
            } else {
                map.insert(task_id, t);
                return Ok(());
            }
        }
    }

    pub(crate) fn remove_task(&self, task_id: usize) -> Result<Rc<SchedTask>, ()> {
        self.task_map
            .borrow_mut()
            .remove(&task_id)
            .ok_or(())
    }
}