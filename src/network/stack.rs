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
use crate::network::protocol::{ProtocolHeaderType, ProtocolMetaData};
use crate::network::socket::NetworkSocket;
use crate::network::tcp::TCPProtocol;
use crate::network::udp::UDPProtocol;
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
    stack_type: ProtocolHeaderType,
    socket_layer: Arc<NetworkSocket>,
    protocol_arp: Arc<ArpProtocol>,
    protocol_ipv4: Arc<IPv4Protocol>,
    protocol_ipv6: Arc<IPv6Protocol>,
    protocol_icmpv4: Arc<ICMPv4Protocol>,
    protocol_icmpv6: Arc<ICMPv6Protocol>,
    protocol_udp: Arc<UDPProtocol>,
    protocol_tcp: Arc<TCPProtocol>,
    protocol_eth: Arc<EthernetProtocol>,
    driver_layer: Arc<NetworkDriver>,
}

impl NetworkStack {
    pub fn new_eth_stack() -> NetworkStack {
        NetworkStack{
            stack_type: ProtocolHeaderType::Ethernet,
            socket_layer: Arc::new(NetworkSocket::new()),
            protocol_arp: Arc::new(ArpProtocol::new()),
            protocol_ipv4: Arc::new(IPv4Protocol::new()),
            protocol_ipv6: Arc::new(IPv6Protocol::new()),
            protocol_icmpv4: Arc::new(ICMPv4Protocol::new()),
            protocol_icmpv6: Arc::new(ICMPv6Protocol::new()),
            protocol_udp: Arc::new(UDPProtocol::new()),
            protocol_tcp: Arc::new(TCPProtocol::new()),
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
        
        let (mut p, mut res) = self.driver_layer.clone().rx(p).await;

        match self.stack_type {
            ProtocolHeaderType::Ethernet => {
                // L2
                (p, res) = self.protocol_eth.clone().decode(p).await;
                let l3_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // L3
                (p, res) = match l3_meta.get_pt() {
                    ProtocolHeaderType::ARP => self.protocol_arp.clone().decode(p).await,
                    ProtocolHeaderType::IPv4 => self.protocol_ipv4.clone().decode(p).await,
                    ProtocolHeaderType::IPv6 => self.protocol_ipv6.clone().decode(p).await,
                    _ => return (p, Err(())),
                };
                let l4_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // l4
                (p, res) = match l4_meta.get_pt() {
                    ProtocolHeaderType::UDP => self.protocol_udp.clone().decode(p).await,
                    ProtocolHeaderType::TCP => self.protocol_tcp.clone().decode(p).await,
                    _ => return (p, Err(())),
                };
                let app_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // socket
                self.socket_layer.clone().rx(p).await
                // (p, Ok(()))
            }
            _ => (p, Err(()))
        }
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