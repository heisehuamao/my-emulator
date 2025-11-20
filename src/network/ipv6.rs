use std::hash::{Hash, Hasher};
use std::net::Ipv6Addr;
use crate::network::protocol::NetworkProtocolMng;

/// CIDR-aware IPv6 key: network address + prefix length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv6Key {
    pub network: [u8; 16], // masked network address
    pub prefix: u8,        // 0..=128
}

impl Hash for Ipv6Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.network.hash(state);
        self.prefix.hash(state);
    }
}

/// IPv6 resource entry: next hop + outbound interface + optional hop limit
#[derive(Debug, Clone)]
pub struct Ipv6Entry {
    pub next_hop: Ipv6Addr,
    pub iface: String,
    pub hop_limit: u8,
}


pub struct Ipv6Protocol {
    pub common: NetworkProtocolMng<Ipv6Key, Ipv6Entry>,
    pub hop_limit_default: u8,
    pub mtu: u32,
}
