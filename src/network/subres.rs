use crate::network::ethernet::MacAddr;

#[derive(Debug, Clone)]
pub(crate) enum SubInfo {
    None,
    Mac(MacAddr),
    // Tunnel(TunnelInfo),
    Vlan(u16),
    // add more as needed
}

impl Default for SubInfo {
    fn default() -> Self {
        SubInfo::None
    }
}