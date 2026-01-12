use crate::network::ipv4::IPv4Addr;
use crate::network::ipv6::IPv6Addr;

pub enum IPAddress {
    IPv4Addr(IPv4Addr),
    IPv6Addr(IPv6Addr),
}