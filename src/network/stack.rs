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
use crate::network::protocol::{ProtocolType, ProtocolMetaData, ProtocolResValue};
use crate::network::socket::{NetworkDomain, NetworkSocketLayer, NetworkType, SockAcceptParam, SockBindParam, SockListenParam, Socket, SocketConnectParam, SocketRecvParam, SocketSendParam};
use crate::network::tcp::TCPProtocol;
use crate::network::udp::UDPProtocol;

pub struct NetworkStack {
    stack_type: ProtocolType,
    socket_layer: Arc<NetworkSocketLayer>,
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
            stack_type: ProtocolType::Ethernet,
            socket_layer: Arc::new(NetworkSocketLayer::new()),
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

    fn add_ipv4_on_ethernet(&self, ip: &IPv4Addr, sub: Option<Arc<dyn Any + Send + Sync>>) -> Result<(), ()> {
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

    fn add_ipv4_internal(&self, ip: &IPv4Addr, sub: Option<Arc<dyn Any + Send + Sync>>) -> Result<(), ()> {
        match self.stack_type {
            ProtocolType::Ethernet => self.add_ipv4_on_ethernet(ip, sub),
            _ => Err(())
        }
    }

    pub fn add_ipv4<'a>(&self, ip: &IPv4Addr, sub_addr: Option<&'a(dyn Any + Send + Sync)>) -> Result<(), ()> {
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

    pub fn add_udp_v4<'a>(&self, ip: &IPv4Addr, port: u16) -> Result<(), ()> {
        // search ipv4
        let search_ipv4_res = self.protocol_ipv4.search_ipv4(ip, ProtocolResValue::default());
        match search_ipv4_res {
            Ok(ipv4_res) => {
                self.protocol_udp.add_udp(port, ipv4_res)
            }
            Err(_) => Err(())
        }
    }

    pub fn mac_show_all(&self) {
        self.protocol_eth.show_all();
    }

    pub fn ipv4_show_all(&self) {
        self.protocol_ipv4.show_all();
    }

    pub fn udp_show_all(&self) {
        self.protocol_udp.show_all();
    }

    pub fn socket_create(&self, dom: NetworkDomain, t: NetworkType, pt: ProtocolType) -> Result<Socket, ()> {
        // Err(())
        self.socket_layer.socket_create()
    }
    
    pub fn socket_bind(&self, id: &Socket, p: &SockBindParam) -> Result<(), ()> {
        self.socket_layer.socket_bind(id, p, self)
    }

    fn socket_listen(&self, id: &Socket, p: &SockListenParam) -> Result<(), ()> {
        self.socket_layer.socket_listen(id, p, self)
    }

    fn socket_accept(&self, id: &Socket, p: &SockAcceptParam) -> Result<(), ()> {
        self.socket_layer.socket_accept(id, p, self)
    }

    fn socket_connect(&self, id: &Socket, pkt: &mut NetworkPacket, p: &SocketConnectParam) -> Result<(), ()> {
        self.socket_layer.socket_connect(id, pkt, self, p)
    }

    fn socket_send(&self, id: &Socket, pkt: &mut NetworkPacket, p: &SocketSendParam) -> Result<(), ()> {
       self.socket_layer.socket_send(id, pkt, self, p)
    }

    fn socket_recv(&self, id: &Socket, pkt: &mut NetworkPacket, p: &SocketRecvParam) -> Result<(), ()> {
        self.socket_layer.socket_recv(id, pkt, self, p)
    }

    fn socket_close(&self, id: &Socket) -> Result<(), ()> {
        self.socket_layer.socket_close(id, self)
    }

    fn socket_destroy(&self, id: &Socket) -> Result<(), ()> {
        self.socket_layer.socket_destroy(id, self)
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
            ProtocolType::Ethernet => {
                // L2
                (p, res) = self.protocol_eth.sync_decode(p);
                let l3_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // L3
                (p, res) = match l3_meta.get_pt() {
                    ProtocolType::ARP => self.protocol_arp.sync_decode(p),
                    ProtocolType::IPv4 => self.protocol_ipv4.sync_decode(p),
                    ProtocolType::IPv6 => self.protocol_ipv6.sync_decode(p),
                    _ => return (p, Err(())),
                };
                let l4_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // l4
                (p, res) = match l4_meta.get_pt() {
                    ProtocolType::UDP => self.protocol_udp.sync_decode(p),
                    ProtocolType::TCP => self.protocol_tcp.sync_decode(p),
                    _ => return (p, Err(())),
                };
                let app_meta = match res {
                    Ok(meta) => meta,
                    _ => return (p, Err(())),
                };
                
                // socket
                // let (p, _) = self.socket_layer.rx();
                (p, Ok(()))
            }
            _ => (p, Err(()))
        }
    }
    async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
        println!("!!!!!!!!!stack tx test. {:?}", p);
        // let (p, res) = self.socket_layer.clone().tx(p).await;
        let (p, res) = self.protocol_eth.sync_encode(p);
        let (p, res) = self.protocol_arp.sync_encode(p);
        let (p, res) = self.protocol_ipv4.sync_encode(p);
        let (p, res) = self.protocol_ipv6.sync_encode(p);
        let (p, res) = self.driver_layer.clone().tx(p).await;
        (p, Ok(()))
    }
}