use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use crate::network::async_modules::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;

pub mod async_modules;
pub mod packet;
mod socket;
mod protocol;
mod driver;
pub mod stack;
mod arp;
mod ipv4;
mod ipv6;
mod icmpv4;
mod icmpv6;
mod ethernet;
//
// pub struct NetworkStack {}
//
// impl NetworkStack {
//     pub fn new_eth_stack() -> NetworkStack {
//         NetworkStack{}
//     }
//
//     // pub fn recv(&self, p: Packet) -> Pin<Box<dyn Future<Output=Result<(), ()>>>> {
//     //     self.rx(p)
//     // }
// }
//
// impl AsyncNetIOModule<Packet> for NetworkStack
// {
//     // type OutputOK = ();
//     // type OutputErr = ();
//     type RxResult = Result<(), ()>;
//     type TxResult = Result<(), ()>;
//
//     fn rx(&self, p: Packet) -> Pin<Box<dyn Future<Output = Self::RxResult> >> {
//         Box::pin(async move {
//             println!("!!!!!!!!!rx test, {:?}", p);
//             Ok(())
//         })
//     }
//     fn tx(&self, p: Packet) -> Pin<Box<dyn Future<Output = Self::TxResult>>> {
//         Box::pin(async move {
//             println!("!!!!!!!!!tx test. {:?}", p);
//             Ok(())
//         })
//     }
// }