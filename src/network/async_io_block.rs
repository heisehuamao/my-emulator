use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// for stack block
pub trait AsyncNetIOBlock<Pkt> {
    type OutputOK;
    type OutputErr;

    fn rx(&self, p: Pkt) -> Pin<Box<dyn Future<Output = Result<Self::OutputOK, Self::OutputErr>> >>;
    fn tx(&self, p: Pkt) -> Pin<Box<dyn Future<Output = Result<Self::OutputOK, Self::OutputErr>> >>;
}