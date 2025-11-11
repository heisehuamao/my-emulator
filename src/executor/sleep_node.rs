use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Duration;
use crate::executor::sched_context::get_sched_context_from_waker;
use crate::executor::task::SchedTask;

pub(crate) struct SleepRet {
    delay: Duration,
    // task: Rc<SchedTask>
}

impl SleepRet {
    // pub(crate) fn new(delay: Duration,
    //                   task: Rc<SchedTask>) -> Self {
    //     Self { delay, task }
    // }
    pub(crate) fn new(delay: Duration) -> Self {
        Self { delay }
    }
}

impl Future for SleepRet {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // push node to sleep ring and pending
        let sched_ctx = get_sched_context_from_waker(cx.waker());
        println!("sched ctx is : {:?}", sched_ctx);
        Poll::Ready(())
    }
}