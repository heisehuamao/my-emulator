use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use crate::network::protocol::NetworkProtocolMng;

/// CIDR-aware key: network address + prefix length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ipv4Key {
    pub network: u32, // network address (masked)
    pub prefix: u8,   // 0..=32
}

impl Hash for Ipv4Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.network.hash(state);
        self.prefix.hash(state);
    }
}

/// IPv4 resource entry: next hop + outbound interface + optional TTL
#[derive(Debug, Clone)]
pub struct Ipv4Entry {
    pub next_hop: Ipv4Addr,
    pub iface: String,
    pub ttl: u64,
}

/// IPv4 protocol that embeds the shared manager and adds IPv4-specific knobs
pub struct Ipv4Protocol {
    pub common: NetworkProtocolMng<Ipv4Key, Ipv4Entry>,
    pub ttl_default: u8,
    pub mtu: u16,
    pub allow_fragmentation: bool,
}