use std::hash::{Hash, Hasher};
use std::net::Ipv6Addr;
use crate::network::ipv4::IPv4Protocol;
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData};

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


pub(crate) struct IPv6Protocol {
    pub common: NetworkProtocolMng<Ipv6Key, Ipv6Entry>,
    pub hop_limit_default: u8,
    pub mtu: u32,
}

impl IPv6Protocol {
    pub(crate) fn new() -> IPv6Protocol {
        IPv6Protocol {
            common: NetworkProtocolMng::new(ProtocolHeaderType::IPv6),
            mtu: 1500,
            hop_limit_default: 64,
        }
    }
}

impl AsyncProtocolModule<NetworkPacket> for IPv6Protocol {
    type EncodeResult = (NetworkPacket, Result<(), ()>);
    type DecodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);

    async fn encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode ipv6 -----");
        (p, Ok(()))
    }

    async fn decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode ipv6 -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::UDP);
        (p, Ok(meta))
    }

    fn sync_encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode ipv6 -----");
        (p, Ok(()))
    }

    fn sync_decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode ipv6 -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::UDP);
        (p, Ok(meta))
    }
}