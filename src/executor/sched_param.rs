use crate::executor::scheduler::Scheduler;

pub(crate) struct SchedParams {
    id: usize,
    name: String,
}

impl SchedParams {
    pub fn new(id: usize, name: String) -> Self {
        Self { id, name }
    }
    
    pub fn get_id(&self) -> usize {
        self.id
    }
}