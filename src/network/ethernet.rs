use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Mutex;
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData};


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MacAddr {
    pub mac: [u8; 6],
}

/// Primary dispatch key: EtherType + optional VLAN ID.
/// - EtherType: e.g., 0x0800 (IPv4), 0x86DD (IPv6), 0x0806 (ARP)
/// - VLAN ID: 0..=4095, None if untagged
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EthKey {
    pub key: MacAddr,
    // pub ethertype: u16,
    // pub vlan: Option<u16>,
}

impl EthKey {
    pub fn new(key: MacAddr) -> EthKey {
        EthKey { key }
    }
}

impl Hash for EthKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key.mac.hash(state);
    }
}

/// Optional MAC-based key for L2 learning/forwarding tables
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct MacKey {
//     pub mac: [u8; 6],        // destination or source MAC
//     pub vlan: Option<u16>,   // to keep per-VLAN separation
// }

/// Ethernet resource: handler metadata or next-hop/port
#[derive(Debug, Clone)]
pub(crate) struct EthEntry {
    mac: MacAddr,        // destination or source MAC
    vlan: Option<u16>,   // to keep per-VLAN separation
//     pub description: String, // e.g., "IPv4 handler" or "Bridge to port 3"
//     pub out_iface: Option<String>,
//     pub priority: u8,        // simple precedence
}

impl EthEntry {
    pub fn new(mac: MacAddr, vlan: Option<u16>) -> EthEntry {
        EthEntry { mac, vlan }
    }
}

pub(crate) struct EthernetProtocol {
    pub common: NetworkProtocolMng<EthKey, EthEntry>,
    // Separate MAC table using the same manager type pattern if desired
    // pub mac_table: Mutex<HashMap<MacKey, EthEntry>>,

    // Ethernet-specific knobs
    pub default_vlan: Option<u16>,
    pub enable_vlan: bool,
}

impl EthernetProtocol {
    pub(crate) fn new() -> EthernetProtocol {
        EthernetProtocol {
            common: NetworkProtocolMng::<EthKey, EthEntry>::new(ProtocolHeaderType::Ethernet),
            // mac_table: Mutex::new(Default::default()),
            default_vlan: None,
            enable_vlan: false,
        }
    }

    pub(crate) fn add_mac(&mut self, mac: MacAddr) -> Result<(), ()> {
        let ky = EthKey::new(mac.clone());
        let ent = EthEntry::new(mac.clone(), None);
        let mut ret = Err(());
        {
            let mut w = self.common.res_write_borrow();
            match (*w).entry(ky) {
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

impl AsyncProtocolModule<NetworkPacket> for EthernetProtocol {
    type EncodeResult = (NetworkPacket, Result<(), ()>);
    type DecodeResult = (NetworkPacket, Result<ProtocolMetaData, ()>);

    async fn encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode ethernet -----");
        (p, Ok(()))
    }

    async fn decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode ethernet -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::IPv4);
        (p, Ok(meta))
    }

    fn sync_encode(&self, p: NetworkPacket) -> Self::EncodeResult {
        println!("----- encode ethernet -----");
        (p, Ok(()))
    }

    fn sync_decode(&self, p: NetworkPacket) -> Self::DecodeResult {
        println!("----- decode ethernet -----");
        let mut meta = ProtocolMetaData::new();
        meta.set_pt(ProtocolHeaderType::IPv4);
        (p, Ok(meta))
    }
}