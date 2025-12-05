use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::network::ethernet::EthKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolHeaderType {
    None,
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

pub(crate) struct NetworkProtocolMng<ProtocolKey, ProtocolRes> {
    header_type: ProtocolHeaderType,
    res_table: Mutex<HashMap<ProtocolKey, ProtocolRes>>,
    res_map: Mutex<HashMap<u64, ProtocolRes>>,
}

impl<ProtocolKey, ProtocolRes> NetworkProtocolMng<ProtocolKey, ProtocolRes> {
    pub(crate) fn new(t: ProtocolHeaderType) -> Self {
        NetworkProtocolMng {
            header_type: t,
            res_table: Mutex::new(HashMap::new()),
            res_map: Mutex::new(HashMap::new()),
        }
    }
}

pub(crate) trait NetworkProtocol {}