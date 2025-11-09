use std::rc::Rc;
use std::task::{RawWaker, RawWakerVTable, Waker};

// Define your own wake trait for Rc
pub trait SchedWake {
    fn wake(self: Rc<Self>);
    fn wake_by_ref(self: &Rc<Self>);
}

pub fn sched_waker_create<T>(rc: Rc<T>) -> Waker
where
    T: SchedWake + 'static,
{
    unsafe fn clone<T: SchedWake>(data: *const ()) -> RawWaker {
        let rc = Rc::from_raw(data as *const T);
        let cloned = rc.clone();
        std::mem::forget(rc); // prevent drop
        RawWaker::new(Rc::into_raw(cloned) as *const (), vtable::<T>())
    }

    unsafe fn wake<T: SchedWake>(data: *const ()) {
        let rc = Rc::from_raw(data as *const T);
        SchedWake::wake(rc);
        // rc consumed
    }

    unsafe fn wake_by_ref<T: SchedWake>(data: *const ()) {
        let rc = Rc::from_raw(data as *const T);
        SchedWake::wake_by_ref(&rc);
        std::mem::forget(rc); // keep ownership
    }

    unsafe fn drop<T: SchedWake>(data: *const ()) {
        let rc = Rc::from_raw(data as *const T);
        std::mem::drop(rc);
    }

    const fn vtable<T: SchedWake>() -> &'static RawWakerVTable {
        &RawWakerVTable::new(
            clone::<T>,
            wake::<T>,
            wake_by_ref::<T>,
            drop::<T>,
        )
    }

    let raw = RawWaker::new(Rc::into_raw(rc) as *const (), vtable::<T>());
    unsafe { Waker::from_raw(raw) }
}