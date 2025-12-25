use std::any::Any;
use std::sync::Arc;
use crate::network::ipv4::IPv4Addr;
use crate::network::arp::ArpProtocol;
use crate::network::module_traits::{AsyncNetIOModule, AsyncProtocolModule};
use crate::network::driver::NetworkDriver;
use crate::network::ethernet::{EthEntry, EthKey, EthernetProtocol, MacAddr};
use crate::network::icmpv4::ICMPv4Protocol;
use crate::network::icmpv6::ICMPv6Protocol;
use crate::network::ipv4::IPv4Protocol;
use crate::network::ipv6::IPv6Protocol;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{ProtocolHeaderType, ProtocolMetaData};
use crate::network::socket::NetworkSocket;
use crate::network::tcp::TCPProtocol;
use crate::network::udp::UDPProtocol;

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

    pub fn add_mac(&self, mac: &MacAddr) -> Result<(), ()> {
        self.protocol_eth.add_mac(mac, None)
    }

    fn add_ipv4_on_ethernet(&self, ip: IPv4Addr, sub: Option<Arc<dyn Any + Send + Sync>>) -> Result<(), ()> {
        let Some(sub_res) = sub else {
            return Err(());
        };

        if let Ok(eth_res) = sub_res.downcast::<EthEntry>() {
            return self.protocol_ipv4.add_ipv4(ip, Some(eth_res));
        } else {
            return Err(());
        }

        Ok(())
    }

    fn add_ipv4_internal(&self, ip: IPv4Addr, sub: Option<Arc<dyn Any + Send + Sync>>) -> Result<(), ()> {
        match self.stack_type {
            ProtocolHeaderType::Ethernet => self.add_ipv4_on_ethernet(ip, sub),
            _ => Err(())
        }
    }

    pub fn add_ipv4<'a>(&self, ip: IPv4Addr, sub_addr: Option<&'a(dyn Any + Send + Sync)>) -> Result<(), ()> {
        let Some(sub_addr_val) = sub_addr else {
            println!("No sub addr for ipv4");
            return Err(());
        };

        if let Some(eth) = sub_addr_val.downcast_ref::<MacAddr>(){
            // search mac
            let search_res = self.protocol_eth.search_mac(eth);
            let ret = match search_res { 
                Ok(mac_res) => {
                    self.protocol_ipv4.add_ipv4(ip, Some(mac_res))
                },
                Err(_) => {
                    println!("Error while searching MAC for IPv4");
                    Err(())
                }
            };
            ret
        } else {
            println!("Sub for ipv4 type error");
            Err(())
        }
    }
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
                (p, res) = self.protocol_eth.sync_decode(p);
                let l3_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // L3
                (p, res) = match l3_meta.get_pt() {
                    ProtocolHeaderType::ARP => self.protocol_arp.sync_decode(p),
                    ProtocolHeaderType::IPv4 => self.protocol_ipv4.sync_decode(p),
                    ProtocolHeaderType::IPv6 => self.protocol_ipv6.sync_decode(p),
                    _ => return (p, Err(())),
                };
                let l4_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // l4
                (p, res) = match l4_meta.get_pt() {
                    ProtocolHeaderType::UDP => self.protocol_udp.sync_decode(p),
                    ProtocolHeaderType::TCP => self.protocol_tcp.sync_decode(p),
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
        let (p, res) = self.protocol_eth.sync_encode(p);
        let (p, res) = self.protocol_arp.sync_encode(p);
        let (p, res) = self.protocol_ipv4.sync_encode(p);
        let (p, res) = self.protocol_ipv6.sync_encode(p);
        let (p, res) = self.driver_layer.clone().tx(p).await;
        (p, Ok(()))
    }
}