use std::hash::{Hash, Hasher};
use std::net::Ipv4Addr;
use crate::network::arp::ArpProtocol;
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData};

/// CIDR-aware key: network address + prefix length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UDPKey {
    pub network: u32, // network address (masked)
    pub prefix: u8,   // 0..=32
}

impl Hash for UDPKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.network.hash(state);
        self.prefix.hash(state);
    }
}

/// IPv4 resource entry: next hop + outbound interface + optional TTL
#[derive(Debug, Clone)]
pub struct UDPEntry {
    pub next_hop: Ipv4Addr,
    pub iface: String,
    pub ttl: u64,
}

/// IPv4 protocol that embeds the shared manager and adds IPv4-specific knobs
pub(crate) struct UDPProtocol {
    pub common: NetworkProtocolMng<UDPKey, UDPEntry>,
    pub ttl_default: u8,
    pub mtu: u16,
    pub allow_fragmentation: bool,
}

impl UDPProtocol {
    pub(crate) fn new() -> UDPProtocol {
        UDPProtocol {
            common: NetworkProtocolMng::new(ProtocolHeaderType::IPv4),
            ttl_default: 64,
            mtu: 1500,
            allow_fragmentation: false,
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
        meta.set_pt(ProtocolHeaderType::Socket);
        (p, Ok(meta))
    }

    fn sync_encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode UDP -----");
        (p, Ok(()))
    }

    fn sync_decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode UDP -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::Socket);
        (p, Ok(meta))
    }
}