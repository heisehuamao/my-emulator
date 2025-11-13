use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Duration;
use crate::executor::runtime::Runtime;
use crate::executor::sched_context::get_sched_context_from_waker;
use crate::executor::task::SchedTask;

pub(crate) struct SleepAsyncNode {
    delayed_to: u64,
    // task: Rc<SchedTask>
}

impl SleepAsyncNode {
    // pub(crate) fn new(delay: Duration,
    //                   task: Rc<SchedTask>) -> Self {
    //     Self { delay, task }
    // }
    pub(crate) fn new(delayed_to: u64) -> Self {
        Self { delayed_to }
    }
}

impl Future for SleepAsyncNode {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // push node to sleep ring and pending
        let sched_ctx = get_sched_context_from_waker(cx.waker());
        println!("sched ctx is : {:?}", sched_ctx);
        
        // match sched_ctx.get_curr_task() {
        //     None => {
        //         Poll::Ready(())
        //     }
        //     Some(t) => {
        //         sched_ctx.get_curr_scheduler().add_to_sleep_ring(self.delayed_to, t);
        //         Poll::Pending
        //     }
        // }
        
        
        // process sleep mode
        // let curr_time_usec = sched_ctx.get_time_usec();
        let curr_time_usec = Runtime::get_time_usec();
        if curr_time_usec < self.delayed_to {
            // put the current task into sleep ring
            match sched_ctx.get_curr_task() {
                None => {
                    panic!("sleep logic error")
                }
                Some(t) => {
                    sched_ctx.get_curr_scheduler().add_to_sleep_ring(self.delayed_to, t);
                }
            }
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}