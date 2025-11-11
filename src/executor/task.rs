use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) struct SchedTask {
    id: Cell<usize>,
    name: String,
    exe_block: Option<RefCell<Pin<Box<dyn Future<Output = ()>>>>>,
}

impl SchedTask {
    pub(crate) fn new(name: String, exe: Pin<Box<dyn Future<Output = ()>>>) -> SchedTask {
        SchedTask { id: Cell::new(0), name, exe_block: Some(RefCell::new(exe)) }
    }
    
    pub(crate) fn set_id(&self, id: usize) {
        self.id.set(id);
    }
    
    pub(crate) fn get_id(&self) -> usize {
        self.id.get()
    }
    
    pub(crate) fn get_task_fut(&self) -> Option<&RefCell<Pin<Box<dyn Future<Output = ()>>>>> {
        match &self.exe_block {
            None => None,
            Some(f) => Some(f),
        }
    }
}

impl Debug for SchedTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task(id:{}, name:{})", self.id.get(), self.name)
    }
}
