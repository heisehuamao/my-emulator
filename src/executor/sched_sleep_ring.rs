use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use crate::executor::runtime::Runtime;
use crate::executor::sched_param::SchedParams;
use crate::executor::task::SchedTask;

struct SleepRingNode {
    delay_to: u64,
    task: Rc<SchedTask>
}

struct SlotVec {
    bucket: RefCell<VecDeque<SleepRingNode>>,
}

impl SlotVec {
    fn new() -> SlotVec {
        SlotVec { bucket: RefCell::new(VecDeque::new()) }
    }

    fn push_node(&self, node: SleepRingNode) {
        self.bucket.borrow_mut().push_back(node);
    }

    fn pop_node(&self) -> Option<SleepRingNode> {
        self.bucket.borrow_mut().pop_front()
    }

    fn pop_all(&self) -> Vec<SleepRingNode> {
        self.bucket.borrow_mut().drain(..).collect()
    }

    fn pop_before_time(&self, curr_time_usec: u64) -> Vec<SleepRingNode> {
        let mut bucket = self.bucket.borrow_mut();

        // Partition into "ready" and "remaining"
        let mut ready = Vec::new();
        let mut remaining = VecDeque::new();

        while let Some(node) = bucket.pop_front() {
            if node.delay_to <= curr_time_usec {
                ready.push(node);
            } else {
                remaining.push_back(node);
            }
        }

        // Put back the not‑yet‑ready nodes
        *bucket = remaining;

        ready
    }
}

pub(crate) struct SchedSleepRing {
    slot_dur: Cell<u64>,
    // exec_time_usec: Cell<u64>,
    exec_slot_idx: Cell<u64>,
    // curr_slot_idx: Cell<u64>,
    max_slot_size: Cell<u64>,
    slots: RefCell<Vec<SlotVec>>,
}

impl SchedSleepRing {
    pub(crate) fn new(n: usize) -> Self {
        let slots = (0..n).map(|_| SlotVec::new()).collect();
        SchedSleepRing {
            slot_dur: Cell::new(100),
            // exec_time_usec: Cell::new(0),
            exec_slot_idx: Cell::new(0),
            // curr_slot_idx: Cell::new(0),
            max_slot_size: Cell::new(n as u64),
            slots: RefCell::new(slots)
        }
    }

    pub(crate) fn add_task_node(&self, delay_to: u64, task: Rc<SchedTask>) {
        let node = SleepRingNode { delay_to, task };
        
        // get idx
        let mut slot_insert_idx = delay_to / self.slot_dur.get();
        let exec_slot_idx = self.exec_slot_idx.get();
        if slot_insert_idx < exec_slot_idx {
            slot_insert_idx = exec_slot_idx;
        }
        
        // insert node
        let insert_idx = slot_insert_idx % self.max_slot_size.get();
        let slots = self.slots.borrow();
        slots[insert_idx as usize].push_node(node);
    }

    fn get_nodes_from_slot(&self, exe_idx: u64, curr_time_usec: u64) -> Vec<SleepRingNode> {
        let real_idx = exe_idx % self.max_slot_size.get();
        let slots = self.slots.borrow();
        if let Some(slot) = slots.get(real_idx as usize) {
            slot.pop_before_time(curr_time_usec)
        } else {
            Vec::new()
        }

    }

    pub(crate) fn get_tasks(&self) -> Vec<Rc<SchedTask>> {
        let curr_time_usec = Runtime::get_time_usec();
        let curr_slot_idx = curr_time_usec / self.slot_dur.get();
        let exec_slot_idx = self.exec_slot_idx.get();

        // println!("Scheduler::get_tasks() exec_slot_idx: {}, curr_slot_idx: {}", exec_slot_idx, curr_slot_idx);
        let mut result = Vec::new();
        for exe_idx in (exec_slot_idx..curr_slot_idx) {
            // get a task from exe_idx
            let mut nodes = self.get_nodes_from_slot(exe_idx, curr_time_usec);
            let mut tasks: Vec<Rc<SchedTask>> = nodes.drain(..).map(|n| n.task).collect();
            result.append(&mut tasks);
            self.exec_slot_idx.set(exe_idx);
        }
        result
    }

    // pub(crate) fn update_curr_time_usec(&self, new_time_usec: u64) {
    //     let old_time_usec = self.exec_time_usec.get();
    //     if new_time_usec > old_time_usec {
    //         let slot_diff = (new_time_usec - old_time_usec) / self.slot_dur.get();
    //         self.exec_time_usec.set(new_time_usec);
    //         self.curr_slot_idx.set(self.curr_slot_idx.get() + slot_diff);
    //     }
    // }
}