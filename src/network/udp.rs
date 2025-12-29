use std::any::Any;
use std::collections::hash_map::Entry;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::sync::Arc;
use crate::network::arp::ArpProtocol;
use crate::network::ethernet::EthEntry;
use crate::network::ipv4::{IPv4Addr, IPv4Entry, IPv4Key};
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolType, ProtocolMetaData, ProtocolResValue};

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct UDPKeyV4 {
//     addr: IPv4Addr,
//     port: u16,
// }

/// CIDR-aware key: network address + prefix length
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UDPKeyV4 {
    addr: IPv4Addr,
    port: u16,
}

impl FromStr for UDPKeyV4 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // split "a.b.c.d/p"
        let (addr_str, port_str) = s
            .split_once('/')
            .ok_or_else(|| "missing '/' in UDPKey".to_string())?;

        // parse prefix
        let port: u16 = port_str
            .parse()
            .map_err(|_| "invalid prefix".to_string())?;

        // parse IPv4 address
        let addr: IPv4Addr = IPv4Addr::from_str(addr_str)
            .map_err(|_| "invalid IPv4 address".to_string())?;

        Ok(UDPKeyV4 { addr, port })
    }
}

impl Display for UDPKeyV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.port)
    }
}

impl Hash for UDPKeyV4 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.val.hash(state);
        self.port.hash(state);
    }
}

impl UDPKeyV4 {
    pub(crate) fn new(addr: &IPv4Addr, port: u16) -> Self {
        UDPKeyV4 { addr: addr.clone(), port }
    }
}

/// IPv4 resource entry: next hop + outbound interface + optional TTL
#[derive(Debug, Clone)]
pub struct UDPEntryV4 {
    addr: IPv4Addr,
    port: u16,
}

impl Display for UDPEntryV4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.port)
    }
}
impl UDPEntryV4 {
    fn new(addr: &IPv4Addr, port: u16) -> Self {
        UDPEntryV4 {
            addr: addr.clone(),
            port
        }
    }
}

/// IPv4 protocol that embeds the shared manager and adds IPv4-specific knobs
pub(crate) struct UDPProtocol {
    pub common_v4: NetworkProtocolMng<UDPKeyV4, Arc<UDPEntryV4>>,
    pub common_v6: NetworkProtocolMng<UDPKeyV4, UDPEntryV4>,
    // pub ttl_default: u8,
    // pub mtu: u16,
    // pub allow_fragmentation: bool,
}

impl UDPProtocol {
    pub(crate) fn new() -> UDPProtocol {
        UDPProtocol {
            common_v4: NetworkProtocolMng::new(ProtocolType::IPv4),
            common_v6: NetworkProtocolMng::new(ProtocolType::IPv4),
            // ttl_default: 64,
            // mtu: 1500,
            // allow_fragmentation: false,
        }
    }

    fn add_udp_v4<'a>(&self, port: u16, addr:&'a IPv4Addr) -> Result<(), ()> {
        let key = UDPKeyV4::new(addr, port);
        let ent = Arc::new(UDPEntryV4::new(addr, port));
        let mut ret = Err(());
        {
            let mut w = self.common_v4.res_write_borrow();
            match (*w).entry(key) {
                Entry::Vacant(v) => {
                    v.insert(ent);
                    ret = Ok(())
                }
                Entry::Occupied(_) => {}
            }
        }
        ret
    }
    
    pub(crate) fn add_udp(&self, port: u16, sub_res: Arc<dyn Any + Send + Sync>) -> Result<(), ()> {
        if let Ok(ipv4_res) = sub_res.downcast::<IPv4Entry>() {
            self.add_udp_v4(port, ipv4_res.addr())
        } else {
            // v6 not supported for the moment
            Err(())
        }
    }

    pub(crate) fn show_all(&self) {
        let r = self.common_v4.res_read_borrow();
        for (_, ent) in r.iter() {
            println!("{}", ent);
        }
    }
}

impl AsyncProtocolModule<NetworkPacket> for UDPProtocol {
    type EncodeResult = (NetworkPacket, Result<(), ()>);
    type DecodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);

    async fn encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode UDP -----");
        (p, Ok(()))
    }

    async fn decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode UDP -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolType::Socket);
        (p, Ok(meta))
    }

    fn sync_encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode UDP -----");
        (p, Ok(()))
    }

    fn sync_decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode UDP -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolType::Socket);
        (p, Ok(meta))
    }
}