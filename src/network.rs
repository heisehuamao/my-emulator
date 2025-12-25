use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;

pub mod module_traits;
pub mod packet;
mod socket;
mod protocol;
mod driver;
pub mod stack;
mod arp;
pub(crate) mod ipv4;
mod ipv6;
mod icmpv4;
mod icmpv6;
pub(crate) mod ethernet;
mod user_app;
mod udp;
mod tcp;
mod subres;
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