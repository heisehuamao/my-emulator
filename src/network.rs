use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use crate::network::async_io_block::AsyncNetIOBlock;
use crate::network::packet::Packet;

pub mod async_io_block;
pub mod packet;

pub struct NetworkStack {}

impl NetworkStack {
    pub fn new_eth_stack() -> NetworkStack {
        NetworkStack{}
    }
}

impl AsyncNetIOBlock<Packet> for NetworkStack
{
    type OutputOK = ();
    type OutputErr = ();

    fn rx(&self, p: Packet) -> Pin<Box<dyn Future<Output = Result<Self::OutputOK, Self::OutputErr>> >> {
        Box::pin(async move {
            println!("!!!!!!!!!rx test, {:?}", p);
            Ok(())
        })
    }
    fn tx(&self, p: Packet) -> Pin<Box<dyn Future<Output = Result<Self::OutputOK, Self::OutputErr>>>> {
        Box::pin(async move {
            println!("!!!!!!!!!tx test. {:?}", p);
            Ok(())
        })
    }
}