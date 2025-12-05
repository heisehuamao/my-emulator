use std::net::Ipv6Addr;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType};

/// ICMPv6 key: destination IPv6 + type + code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icmpv6Key {
    pub dst: Ipv6Addr,
    pub icmp_type: u8, // e.g. 128 = Echo Request, 129 = Echo Reply
    pub icmp_code: u8, // subtype code (often 0 for echo)
}

/// ICMPv6 resource: reply template or handler metadata
#[derive(Debug, Clone)]
pub struct Icmpv6Entry {
    pub description: String,
    pub hop_limit: u8,          // optional control
    pub identifier: Option<u16>, // echo identifier (if relevant)
    pub sequence: Option<u16>,   // echo sequence (if relevant)
}

pub struct ICMPv6Protocol {
    pub common: NetworkProtocolMng<Icmpv6Key, Icmpv6Entry>,
    pub default_hop_limit: u8,
}

impl ICMPv6Protocol {
    pub(crate) fn new() -> ICMPv6Protocol {
        ICMPv6Protocol {
            common: NetworkProtocolMng::new(ProtocolHeaderType::ICMPv6),
            default_hop_limit: 0,
        }
    }
}
