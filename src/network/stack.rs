use std::any::Any;
use std::pin::Pin;
use crate::network::arp::ArpProtocol;
use crate::network::async_modules::AsyncNetIOModule;
use crate::network::driver::NetworkDriver;
use crate::network::ethernet::EthernetProtocol;
use crate::network::icmpv4::Icmpv4Protocol;
use crate::network::icmpv6::Icmpv6Protocol;
use crate::network::ipv4::Ipv4Protocol;
use crate::network::ipv6::Ipv6Protocol;
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
    socket_layer: NetworkSocket,
    protocol_arp: ArpProtocol,
    protocol_ipv4: Vec<Ipv4Protocol>,
    protocol_ipv6: Vec<Ipv6Protocol>,
    protocol_icmpv4: Vec<Icmpv4Protocol>,
    protocol_icmpv6: Vec<Icmpv6Protocol>,
    protocol_eth: EthernetProtocol,
    driver_layer: NetworkDriver,
}

impl NetworkStack {
    pub fn new_eth_stack() -> NetworkStack {
        NetworkStack{ 
            socket_layer: NetworkSocket {},
            protocol_arp: ArpProtocol::new(),
            protocol_ipv4: vec![],
            protocol_ipv6: vec![],
            protocol_icmpv4: vec![],
            protocol_icmpv6: vec![],
            protocol_eth: EthernetProtocol::new(),
            driver_layer: NetworkDriver {}
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
    type RxResult = Result<(), ()>;
    type TxResult = Result<(), ()>;

    fn rx(&self, p: NetworkPacket) -> Pin<Box<dyn Future<Output = Self::RxResult> >> {
        Box::pin(async move {
            println!("!!!!!!!!!rx test, {:?}", p);
            Ok(())
        })
    }
    fn tx(&self, p: NetworkPacket) -> Pin<Box<dyn Future<Output = Self::TxResult>>> {
        Box::pin(async move {
            println!("!!!!!!!!!tx test. {:?}", p);
            Ok(())
        })
    }
}