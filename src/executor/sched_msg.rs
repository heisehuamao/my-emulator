use std::fmt::{Debug, Formatter};
use std::pin::Pin;

use std::future::Future;


pub(crate) type AsyncTaskFnBox = Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;
pub(crate) struct SchedMsg {
    cmd: String,
    // task_func: Option<Box<dyn Fn(String) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>>,
    task_func: Option<AsyncTaskFnBox>
}

impl SchedMsg {
    pub(crate) fn new(cmd: String, task_func: Option<AsyncTaskFnBox>) -> Self {
        Self { cmd, task_func }
    }

    pub(crate) fn get_cmd(&self) -> &str {
        &self.cmd
    }

    pub(crate) fn get_task_func(&mut self) -> Option<AsyncTaskFnBox> {
        self.task_func.take()
    }
}

impl Debug for SchedMsg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SchedMsg({})", self.cmd)
    }
}