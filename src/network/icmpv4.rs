use std::net::Ipv4Addr;
use crate::network::protocol::NetworkProtocolMng;

/// ICMPv4 key: destination IPv4 + type + code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icmpv4Key {
    pub dst: Ipv4Addr,
    pub icmp_type: u8, // e.g. 8 = Echo Request, 0 = Echo Reply
    pub icmp_code: u8, // subtype code
}

/// ICMPv4 resource: reply template or handler metadata
#[derive(Debug, Clone)]
pub struct Icmpv4Entry {
    pub description: String,
    pub ttl: u64,           // optional expiration/control
    pub identifier: Option<u16>, // echo identifier (if relevant)
    pub sequence: Option<u16>,   // echo sequence (if relevant)
}


pub struct Icmpv4Protocol {
    pub common: NetworkProtocolMng<Icmpv4Key, Icmpv4Entry>,
    pub default_ttl: u64,
}
