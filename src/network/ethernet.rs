use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use crate::network::module_traits::AsyncProtocolModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType, ProtocolMetaData};


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MacAddr {
    pub mac: [u8; 6],
}

// Implement FromStr for safe parsing
impl FromStr for MacAddr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(|c| c == ':' || c == '-').collect();
        if parts.len() != 6 {
            return Err("Invalid MAC address format".into());
        }

        let mut mac = [0u8; 6];
        for (i, part) in parts.iter().enumerate() {
            mac[i] = u8::from_str_radix(part, 16)
                .map_err(|_| format!("Invalid hex value: {}", part))?;
        }

        Ok(MacAddr { mac })
    }
}

// Implement Display for formatting back to string
impl Display for MacAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.mac[0], self.mac[1], self.mac[2],
            self.mac[3], self.mac[4], self.mac[5]
        )
    }
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
    pub fn new(key_ref: &MacAddr) -> EthKey {
        EthKey { key: key_ref.clone() }
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
    sub: Option<Arc<dyn Any + Send + Sync>>,
}

impl EthEntry {
    pub fn new(mac: MacAddr, vlan: Option<u16>, sub: Option<Arc<dyn Any + Send + Sync>>) -> EthEntry {
        EthEntry { mac, vlan, sub: None }
    }
}

pub(crate) struct EthernetProtocol {
    pub common: NetworkProtocolMng<EthKey, Arc<EthEntry>>,
    // Separate MAC table using the same manager type pattern if desired
    // pub mac_table: Mutex<HashMap<MacKey, EthEntry>>,

    // Ethernet-specific knobs
    pub default_vlan: Option<u16>,
    pub enable_vlan: bool,
}

impl EthernetProtocol {
    pub(crate) fn new() -> EthernetProtocol {
        EthernetProtocol {
            common: NetworkProtocolMng::<EthKey, Arc<EthEntry>>::new(ProtocolHeaderType::Ethernet),
            // mac_table: Mutex::new(Default::default()),
            default_vlan: None,
            enable_vlan: false,
        }
    }

    pub(crate) fn add_mac(&self, mac: &MacAddr, sub: Option<Arc<dyn Any + Send + Sync>>) -> Result<(), ()> {
        let ky = EthKey::new(&mac);
        let ent = Arc::new(EthEntry::new(mac.clone(), None, sub));
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
    
    pub(crate) fn search_mac(&self, mac: &MacAddr) -> Result<Arc<EthEntry>, ()> {
        let key = EthKey::new(&mac);
        let mut r = self.common.res_read_borrow();
        match r.get(&key).map(Arc::clone) {
            Some(ent) => Ok(ent),
            None => {
                println!("No entry found for {:?}", mac);
                Err(())
            },
        }
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