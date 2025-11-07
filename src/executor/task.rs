use std::fmt::{Debug, Formatter};
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) struct SchedTask {
    name: String,
    exe_block: Option<Pin<Box<dyn Future<Output = ()>>>>,
}

impl SchedTask {
    pub fn new(name: String) -> SchedTask {
        SchedTask { name, exe_block: None }
    }
}

impl Debug for SchedTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task({})", self.name)
    }
}
