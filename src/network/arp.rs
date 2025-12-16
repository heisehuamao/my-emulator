use crate::network::ethernet::EthernetProtocol;
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ArpKey {
    pub ip_addr: [u8; 4],   // IPv4 address
}

#[derive(Debug)]
pub(crate) struct ArpEntry {
    pub mac_addr: [u8; 6],  // MAC address
    pub ttl: u64,           // time-to-live or expiration
}

pub(crate) struct ArpProtocol {
    common: NetworkProtocolMng<ArpKey, ArpEntry>,
    // ARP-specific fields
    cache_timeout: u64,
}

impl ArpProtocol {
    pub(crate) fn new() -> ArpProtocol {
        ArpProtocol {
            common: NetworkProtocolMng::new(ProtocolHeaderType::ARP),
            cache_timeout: 0,
        }
    }
}

impl AsyncProtocolModule<NetworkPacket> for ArpProtocol {
    type EncodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);
    type DecodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);

    async fn encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode arp -----");
        (p, Ok(ProtocolMetaData::new()))
    }

    async fn decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode arp -----");
        (p, Ok(ProtocolMetaData::new()))
    }

    fn sync_encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode arp -----");
        (p, Ok(ProtocolMetaData::new()))
    }

    fn sync_decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode arp -----");
        (p, Ok(ProtocolMetaData::new()))
    }
}
