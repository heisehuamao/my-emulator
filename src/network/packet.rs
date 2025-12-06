use std::cell::{Cell, Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::network::protocol::ProtocolHeaderType;

struct FixedPacketBuffer {
    w_idx: Cell<usize>,
    // r_idx: Cell<usize>,
    buffer: RefCell<Vec<u8>>,
}

impl FixedPacketBuffer {
    fn new(sz: usize) -> FixedPacketBuffer {
        FixedPacketBuffer {
            w_idx: Cell::new(0),
            // r_idx: Cell::new(0),
            buffer: RefCell::new(vec![0; sz]),
        }
    }

    fn reset(&self) {
        self.w_idx.set(0);
    }

    fn free_space(&self) -> usize {
        self.buffer.borrow().len() - self.w_idx.get()
    }

    fn data_len(&self) -> usize {
        self.w_idx.get()
    }

    fn push_data(&self, data: &[u8]) -> Result<(), ()> {
        let buffer_len = self.buffer.borrow().len();
        let w_idx = self.w_idx.get();
        let data_len = data.len();
        if w_idx >= buffer_len || w_idx + data_len > buffer_len {
            return Err(());
        }
        let free_space = buffer_len - w_idx;
        let mut s = &mut self.buffer.borrow_mut()[w_idx..w_idx + data_len];
        s.copy_from_slice(data);
        Ok(())
    }

    fn borrow_excerpt(&self, offset: usize, len: usize) -> Result<Ref<[u8]>, ()> {
        let w_len = self.w_idx.get();
        if offset + len > w_len {
            return Err(());
        }

        let slice_ref = Ref::map(self.buffer.borrow(),
                                 |v| &v[offset..offset + len]);
        // let slice_ref = &ref_s[offset..offset + len];
        Ok(slice_ref)
    }
}

struct PacketFragHeader {
    hdr_type: Cell<ProtocolHeaderType>,
    hdr_offset: Cell<u32>,
    hdr_len: Cell<u32>,
}

impl PacketFragHeader {
    pub(crate) fn new(hdr_type: ProtocolHeaderType) -> PacketFragHeader {
        PacketFragHeader {
            hdr_type: Cell::new(hdr_type),
            hdr_offset: Cell::new(0),
            hdr_len: Cell::new(0),
        }
    }

}

struct PacketFrag {
    hdrs: RefCell<Vec<PacketFragHeader>>,
    // w_offset: Cell<u32>,
    // r_offset: Cell<u32>,
    buffer: RefCell<FixedPacketBuffer>,
}

impl PacketFrag {
    pub(crate) fn new(sz: usize) -> PacketFrag {
        PacketFrag {
            hdrs: RefCell::new(Vec::new()),
            // w_offset: Cell::new(0),
            // r_offset: Cell::new(0),
            buffer: RefCell::new(FixedPacketBuffer::new(sz)),
        }
    }

    pub(crate) fn push_data(&self, data: &[u8]) -> Result<(), ()> {
        self.buffer.borrow().push_data(data)
    }


    pub(crate) fn data_len(&self) -> usize {
        self.buffer.borrow().data_len()
    }
}

impl Debug for PacketFrag {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "PacketFrag")
    }
}

pub struct NetworkPacket {
    // size: Cell<usize>,
    app_id: Cell<usize>,
    sock_id: Cell<usize>,
    frags: RefCell<Vec<PacketFrag>>,
}

impl NetworkPacket {
    pub fn new() -> NetworkPacket {
        NetworkPacket {
            // size: Cell::new(0),
            app_id: Cell::new(0),
            sock_id: Cell::new(0),
            frags: RefCell::new(Vec::new())
        }
    }

    fn push_data_to_new_frag(&self, data: &[u8]) -> Result<(), ()> {
        let frag = PacketFrag::new(1514);
        frag.push_data(data)?;
        self.frags.borrow_mut().push(frag);
        Ok(())
    }
    
    pub fn push_data(&self, data: &[u8]) -> Result<(), ()> {
        let mut res = Err(());
        if let Some(frag) = self.frags.borrow().last() {
            res = frag.push_data(data);
        }
        
        match res {
            Ok(()) => Ok(()),
            Err(()) => self.push_data_to_new_frag(data),
        }
    }

    pub fn data_len(&self) -> usize {
        let mut sz = 0;
        for p in self.frags.borrow().iter() {
            sz = sz + p.data_len();
        }
        sz
    }
}

impl Debug for NetworkPacket {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Packet")
            // `Cell` gives us `get()`
            .field("size", &self.data_len())
            // `RefCell` requires borrow; we pass a reference to the Vec
            .field("frags", &self.frags.borrow())
            .finish()
    }
}