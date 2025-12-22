use std::any::Any;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use std::sync::Arc;
use crate::network::arp::ArpProtocol;
use crate::network::ethernet::{EthEntry, EthKey, MacAddr};
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData, ProtocolResValue};
use crate::network::subres::SubInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IPv4Addr {
    pub val: [u8; 6],
}

/// CIDR-aware key: network address + prefix length
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IPv4Key {
    pub addr: IPv4Addr, // network address (masked)
    pub sub: ProtocolResValue,
}

impl IPv4Key {
    pub fn new(addr: IPv4Addr, sub: ProtocolResValue) -> Self {
        IPv4Key { addr, sub }
    }
}

impl Hash for IPv4Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr.val.hash(state);
        self.sub.hash(state);
    }
}

/// IPv4 resource entry: next hop + outbound interface + optional TTL
#[derive(Debug, Clone)]
pub(crate) struct IPv4Entry {
    addr: IPv4Addr, // network address (masked)
    mask: u8,
    mtu: u16,
    sub: Option<Arc<dyn Any + Send + Sync>>,
}

impl IPv4Entry {
    pub fn new(addr: IPv4Addr, mtu: u16, sub: Option<Arc<dyn Any + Send + Sync>>) -> Self {
        IPv4Entry {
            addr,
            mask: 0,
            mtu,
            sub,
        }
    }
}

/// IPv4 protocol that embeds the shared manager and adds IPv4-specific knobs
pub(crate) struct IPv4Protocol {
    pub common: NetworkProtocolMng<IPv4Key, Arc<IPv4Entry>>,
    pub ttl_default: u8,
    pub mtu: u16,
    pub allow_fragmentation: bool,
}

impl IPv4Protocol {
    pub(crate) fn new() -> IPv4Protocol {
        IPv4Protocol {
            common: NetworkProtocolMng::new(ProtocolHeaderType::IPv4),
            ttl_default: 64,
            mtu: 1500,
            allow_fragmentation: false,
        }
    }

    pub(crate) fn add_ipv4(&self, addr: IPv4Addr, sub: Option<Arc<dyn Any + Send + Sync>>,) -> Result<(), ()> {
        let key = IPv4Key::new(addr.clone(), ProtocolResValue::default());
        let ent = Arc::new(IPv4Entry::new(addr.clone(), 1000, sub));
        let mut ret = Err(());
        {
            let mut w = self.common.res_write_borrow();
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
}

impl AsyncProtocolModule<NetworkPacket> for IPv4Protocol {
    type EncodeResult = (NetworkPacket, Result<(), ()>);
    type DecodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);

    async fn encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode ipv4 -----");
        (p, Ok(()))
    }

    async fn decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode ipv4 -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::UDP);
        (p, Ok(meta))
    }

    fn sync_encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode ipv4 -----");
        (p, Ok(()))
    }

    fn sync_decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode ipv4 -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::UDP);
        (p, Ok(meta))
    }
}