use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
pub struct SchedTask {
    name: String,
}

impl SchedTask {
    pub fn new(name: String) -> SchedTask {
        SchedTask { name }
    }
}

impl Future for SchedTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}