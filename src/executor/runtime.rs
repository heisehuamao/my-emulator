use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
use crate::executor::scheduler::Scheduler;
use crate::executor::sleep_async::SleepAsyncNode;
use crate::executor::task::SchedTask;

thread_local! {
    static CURR_SCHEDULER: RefCell<Option<Rc<Scheduler>>> = RefCell::new(None);
    static CNT_TEST: Cell<usize> = Cell::new(0);
    static CURR_TIME_USEC: Cell<u64> = Cell::new(0);
    static CURR_RUNNING_TASK: Cell<Option<Rc<SchedTask>>> = Cell::new(None);
}

pub struct Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sleep(dur: Duration) -> SleepAsyncNode {
        let res = Self::get_scheduler();
        match res {
            Some(sched) => {
                let dur_usec = dur.as_micros() as u64;
                sched.sched_sleep(dur_usec)
            }
            None => {
                panic!("Scheduler not running");
            }
        }
    }
    
    pub(crate) fn get_time_usec() -> u64 {
        CURR_TIME_USEC.get()
    }
    
    pub(crate) fn set_time_usec(updated_time_usec: u64) {
        let curr_time_usec = CURR_TIME_USEC.get();
        if updated_time_usec < curr_time_usec {
            panic!("Time used exceeded");
        }
        CURR_TIME_USEC.set(updated_time_usec);
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
}

