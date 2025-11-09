use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) struct SchedTask {
    name: String,
    exe_block: Option<RefCell<Pin<Box<dyn Future<Output = ()>>>>>,
}

impl SchedTask {
    pub fn new(name: String, exe: Pin<Box<dyn Future<Output = ()>>>) -> SchedTask {
        SchedTask { name, exe_block: Some(RefCell::new(exe)) }
    }
    
    pub fn get_task_fut(&self) -> Option<&RefCell<Pin<Box<dyn Future<Output = ()>>>>> {
        match &self.exe_block {
            None => None,
            Some(f) => Some(f),
        }
    }
}

impl Debug for SchedTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task({})", self.name)
    }
}
