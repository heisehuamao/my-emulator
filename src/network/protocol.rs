use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Mutex, RwLock, RwLockWriteGuard};
use crate::network::ethernet::EthKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolHeaderType {
    None,
    Socket,
    Ethernet,
    ARP,
    IPv4,
    IPv6,
    UDP,
    TCP,
    ICMPv4,
    ICMPv6,
}

impl ProtocolHeaderType {
    pub fn code(&self) -> u16 {
        match self {
            ProtocolHeaderType::Ethernet => 1,
            ProtocolHeaderType::ARP      => 0xaa,
            ProtocolHeaderType::IPv4     => 0x0800, // typical EtherType for IPv4
            ProtocolHeaderType::IPv6     => 0x86DD, // typical EtherType for IPv6
            ProtocolHeaderType::UDP      => 17,     // IP protocol number
            ProtocolHeaderType::TCP      => 6,      // IP protocol number
            ProtocolHeaderType::ICMPv4    => 0xa4,
            ProtocolHeaderType::ICMPv6    => 0xa6,
            ProtocolHeaderType::None    => 0,
            _ => 0
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResValue {
    pub t: ProtocolHeaderType,   // 0..=32
    pub v: Option<Vec<u8>>,
}

impl Hash for ProtocolResValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.t.code().hash(state);
        match &self.v {
            Some(v) => {v.hash(state);},
            None => {}
        }
    }
}

pub(crate) struct NetworkProtocolMng<ProtocolKey, ProtocolRes> {
    header_type: ProtocolHeaderType,
    res_table: RwLock<HashMap<ProtocolKey, ProtocolRes>>,
    res_map: RwLock<HashMap<u64, ProtocolRes>>,
}

impl<ProtocolKey, ProtocolRes> NetworkProtocolMng<ProtocolKey, ProtocolRes> {
    pub(crate) fn new(t: ProtocolHeaderType) -> Self {
        NetworkProtocolMng {
            header_type: t,
            res_table: RwLock::new(HashMap::new()),
            res_map: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn res_write_borrow(&self) -> RwLockWriteGuard<HashMap<ProtocolKey, ProtocolRes>> {
        let res = self.res_table.write();
        res.unwrap()
    }
}

pub(crate) trait NetworkProtocol {}

pub(crate) struct ProtocolMetaData {
    pt: ProtocolHeaderType,
}

impl ProtocolMetaData {
    pub(crate) fn new() -> Self {
        ProtocolMetaData { pt: ProtocolHeaderType::None }
    }

    pub(crate) fn set_pt(&mut self, p: ProtocolHeaderType) {
        self.pt = p;
    }

    pub(crate) fn get_pt(&self) -> ProtocolHeaderType {
        self.pt
    }
}