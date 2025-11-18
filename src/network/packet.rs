use std::cell::Cell;


#[derive(Debug)]
pub struct Packet {
    size: Cell<usize>
}


impl Packet {
    pub fn new(size: usize) -> Packet {
        Packet { size: Cell::new(size) }
    }
}