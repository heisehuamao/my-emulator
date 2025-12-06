use std::any::Any;
use std::pin::Pin;
use std::sync::Arc;
use crate::network::arp::ArpProtocol;
use crate::network::module_traits::{AsyncNetIOModule, AsyncProtocolModule};
use crate::network::driver::NetworkDriver;
use crate::network::ethernet::EthernetProtocol;
use crate::network::icmpv4::ICMPv4Protocol;
use crate::network::icmpv6::ICMPv6Protocol;
use crate::network::ipv4::IPv4Protocol;
use crate::network::ipv6::IPv6Protocol;
use crate::network::packet::NetworkPacket;
use crate::network::socket::NetworkSocket;


/*
multiple levels of IPv4
          +-------------------+
          |     Ethernet      |
          +---------+---------+
                    |
                    | EtherType: 0x0800 (IPv4)
                    v
          +-------------------+
          |       IPv4        |
          |  (Outer Header)   |
          | Protocol: 4 (IPv4)|
          +---------+---------+
                    |
                    |
                    v
          +-------------------+
          |       IPv4        |
          |  (Inner Header)   |
          | Protocol: 1 (ICMPv4) OR 4 (IPv4) OR 6 (TCP) OR 17 (UDP)
          +---------+---------+
          /    /    \    \
         /    /      \    \
        v    v        v    v
 +-------+ +-------+ +-------+ +-------+
 | ICMPv4| |  IPv4 | |  TCP  | |  UDP  |
 |       | | (Inner| |       | |       |
 |       | | most) | |       | |       |
 +-------+ +-------+ +-------+ +-------+
               |        |         |
               |        |         |
               v        v         v
             +-------+ +-------+ +-------+
             | ICMPv4| | HTTP  | | DNS   |
             +-------+ +-------+ +-------+
 */
pub struct NetworkStack {
    socket_layer: Arc<NetworkSocket>,
    protocol_arp: Arc<ArpProtocol>,
    protocol_ipv4: Arc<IPv4Protocol>,
    protocol_ipv6: Arc<IPv6Protocol>,
    protocol_icmpv4: Arc<ICMPv4Protocol>,
    protocol_icmpv6: Arc<ICMPv6Protocol>,
    protocol_eth: Arc<EthernetProtocol>,
    driver_layer: Arc<NetworkDriver>,
}

impl NetworkStack {
    pub fn new_eth_stack() -> NetworkStack {
        NetworkStack{ 
            socket_layer: Arc::new(NetworkSocket::new()),
            protocol_arp: Arc::new(ArpProtocol::new()),
            protocol_ipv4: Arc::new(IPv4Protocol::new()),
            protocol_ipv6: Arc::new(IPv6Protocol::new()),
            protocol_icmpv4: Arc::new(ICMPv4Protocol::new()),
            protocol_icmpv6: Arc::new(ICMPv6Protocol::new()),
            protocol_eth: Arc::new(EthernetProtocol::new()),
            driver_layer: Arc::new(NetworkDriver {})
        }
    }

    // pub fn recv(&self, p: Packet) -> Pin<Box<dyn Future<Output=Result<(), ()>>>> {
    //     self.rx(p)
    // }
}

impl AsyncNetIOModule<NetworkPacket> for NetworkStack
{
    // type OutputOK = ();
    // type OutputErr = ();
    type RxResult = (NetworkPacket, Result<(), ()>);
    type TxResult = (NetworkPacket, Result<(), ()>);

    // fn rx(self: Arc<Self>, p: NetworkPacket) -> Pin<Box<dyn Future<Output = Self::RxResult> >> {
    async fn rx(self: Arc<Self>, p: NetworkPacket) -> Self::RxResult {
        
        println!("!!!!!!!!!stack rx test, {:?}", p);
        let (p, res) = self.driver_layer.clone().rx(p).await;
        let (p, res) = self.protocol_eth.clone().decode(p).await;
        let (p, res) = self.protocol_arp.clone().decode(p).await;
        let (p, res) = self.protocol_ipv4.clone().decode(p).await;
        let (p, res) = self.protocol_ipv6.clone().decode(p).await;
        let (p, res) = self.socket_layer.clone().rx(p).await;
        (p, Ok(()))
    }
    async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
        println!("!!!!!!!!!stack tx test. {:?}", p);
        let (p, res) = self.socket_layer.clone().tx(p).await;
        let (p, res) = self.protocol_eth.clone().encode(p).await;
        let (p, res) = self.protocol_arp.clone().encode(p).await;
        let (p, res) = self.protocol_ipv4.clone().encode(p).await;
        let (p, res) = self.protocol_ipv6.clone().encode(p).await;
        let (p, res) = self.driver_layer.clone().tx(p).await;
        (p, Ok(()))
    }
}