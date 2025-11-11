use std::rc::Rc;
use crate::executor::task::SchedTask;

struct SlotVec {
    bucket: Vec<Rc<SchedTask>>,
}

impl SlotVec {
    fn new() -> SlotVec {
        SlotVec { bucket: Vec::new() }
    }
}

pub(crate) struct SchedSleepRing {
    slots: Vec<SlotVec>,
}

impl SchedSleepRing {
    pub(crate) fn new(n: usize) -> Self {
        let slots = (0..n).map(|_| SlotVec::new()).collect();
        SchedSleepRing { slots }
    }
}