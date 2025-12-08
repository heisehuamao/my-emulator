use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use crate::network::arp::ArpProtocol;
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData};

/// CIDR-aware key: network address + prefix length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TCPKey {
    pub network: u32, // network address (masked)
    pub prefix: u8,   // 0..=32
}

impl Hash for TCPKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.network.hash(state);
        self.prefix.hash(state);
    }
}

/// IPv4 resource entry: next hop + outbound interface + optional TTL
#[derive(Debug, Clone)]
pub struct TCPEntry {
    pub next_hop: Ipv4Addr,
    pub iface: String,
    pub ttl: u64,
}

/// IPv4 protocol that embeds the shared manager and adds IPv4-specific knobs
pub(crate) struct TCPProtocol {
    pub common: NetworkProtocolMng<TCPKey, TCPEntry>,
    pub ttl_default: u8,
    pub mtu: u16,
    pub allow_fragmentation: bool,
}

impl TCPProtocol {
    pub(crate) fn new() -> TCPProtocol {
        TCPProtocol {
            common: NetworkProtocolMng::new(ProtocolHeaderType::IPv4),
            ttl_default: 64,
            mtu: 1500,
            allow_fragmentation: false,
        }
    }
}

impl AsyncProtocolModule<NetworkPacket> for TCPProtocol {
    type EncodeResult = (NetworkPacket, Result<(), ()>);
    type DecodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);

    async fn encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode TCP -----");
        (p, Ok(()))
    }

    async fn decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode TCP -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::Socket);
        (p, Ok(meta))
    }
}