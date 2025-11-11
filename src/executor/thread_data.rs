use std::cell::{Cell, RefCell};
use std::rc::Rc;
use crate::executor::scheduler::Scheduler;

thread_local! {
    static CURR_SCHEDULER: RefCell<Option<Rc<Scheduler>>> = RefCell::new(None);
    static CNT_TEST: Cell<usize> = Cell::new(0);
}

pub(crate) fn set_scheduler(sched: &Rc<Scheduler>) {
    CNT_TEST.with(|a| {
        a.set(100);
        println!("------ thread {:?} init, value: {}", std::thread::current().id(), a.get());
    });
    
    CURR_SCHEDULER.with(|curr| {
        curr.replace(Some(sched.clone()));
    })
}

pub(crate) fn get_scheduler() -> Option<Rc<Scheduler>>  {

    CNT_TEST.with(|a| {
        a.set(a.get() + 1);
        println!("------ thread {:?} running, value: {}", std::thread::current().id(), a.get());
    });
    
    CURR_SCHEDULER.with(|curr| {
        curr.borrow()
            .clone()
    })
}

pub(crate) fn clear_scheduler() {
    CURR_SCHEDULER.with(|curr| {
        CNT_TEST.with(|a| {
            a.set(a.get() + 1);
            println!("------ thread {:?} end, value: {}", std::thread::current().id(), a.get());
        });
        curr.replace(None);
    })
}