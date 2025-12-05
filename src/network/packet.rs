use std::cell::{Cell, RefCell};
use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::network::protocol::ProtocolHeaderType;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum PacketFragHeaderType {
//     Ethernet,
//     ARP,
//     IPv4,
//     IPv6,
//     UDP,
//     TCP,
// }

struct PacketFragHeader {
    hdr_type: Cell<ProtocolHeaderType>,
    hdr_len: Cell<u32>,
}
struct PacketFrag {
    hdrs: Vec<PacketFragHeader>,
    buffer: RefCell<Vec<u8>>,
}

impl PacketFrag {

}

impl Debug for PacketFrag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "PacketFrag")
    }
}

pub struct NetworkPacket {
    size: Cell<usize>,
    app_id: Cell<usize>,
    sock_id: Cell<usize>,
    frags: RefCell<Vec<PacketFrag>>,
}


impl NetworkPacket {
    pub fn new() -> NetworkPacket {
        NetworkPacket {
            size: Cell::new(0),
            app_id: Cell::new(0),
            sock_id: Cell::new(0),
            frags: RefCell::new(Vec::new())
        }
    }
}

impl Debug for NetworkPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packet")
            // `Cell` gives us `get()`
            .field("size", &self.size.get())
            // `RefCell` requires borrow; we pass a reference to the Vec
            .field("frags", &self.frags.borrow())
            .finish()
    }
}