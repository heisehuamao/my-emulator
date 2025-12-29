use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::network::ethernet::EthKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtocolType {
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

impl ProtocolType {
    pub fn code(&self) -> u16 {
        match self {
            ProtocolType::Ethernet => 1,
            ProtocolType::ARP      => 0xaa,
            ProtocolType::IPv4     => 0x0800, // typical EtherType for IPv4
            ProtocolType::IPv6     => 0x86DD, // typical EtherType for IPv6
            ProtocolType::UDP      => 17,     // IP protocol number
            ProtocolType::TCP      => 6,      // IP protocol number
            ProtocolType::ICMPv4    => 0xa4,
            ProtocolType::ICMPv6    => 0xa6,
            ProtocolType::None    => 0,
            _ => 0
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolResValue {
    pub t: ProtocolType,   // 0..=32
    pub v: Option<Vec<u8>>,
}

impl Default for ProtocolResValue {
    fn default() -> Self {
        ProtocolResValue {
            t: ProtocolType::None,
            v: None
        }
    }
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
    header_type: ProtocolType,
    res_table: RwLock<HashMap<ProtocolKey, ProtocolRes>>,
    res_map: RwLock<HashMap<u64, ProtocolRes>>,
}

impl<ProtocolKey, ProtocolRes> NetworkProtocolMng<ProtocolKey, ProtocolRes> {
    pub(crate) fn new(t: ProtocolType) -> Self {
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

    pub(crate) fn res_read_borrow(&self) -> RwLockReadGuard<HashMap<ProtocolKey, ProtocolRes>> {
        let res = self.res_table.read();
        res.unwrap()
    }
}

pub(crate) trait NetworkProtocol {}

pub(crate) struct ProtocolMetaData {
    pt: ProtocolType,
}

impl ProtocolMetaData {
    pub(crate) fn new() -> Self {
        ProtocolMetaData { pt: ProtocolType::None }
    }

    pub(crate) fn set_pt(&mut self, p: ProtocolType) {
        self.pt = p;
    }

    pub(crate) fn get_pt(&self) -> ProtocolType {
        self.pt
    }
}