use modular_bitfield::{bitfield};

#[bitfield]
#[derive(Debug)]
pub struct Flags {
    val: u8,
}

fn main() {}