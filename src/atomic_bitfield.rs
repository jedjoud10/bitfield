use std::{fmt::{Debug, Display}, sync::atomic::{AtomicU64, Ordering}};

use crate::data::BitfieldData;

// An atomic bitfield that uses a U64
#[derive(Default)]
pub struct AtomicBitfieldU64 {
    bitfield: AtomicU64,
}
impl AtomicBitfieldU64 {
    // Create a new empty bitfield
    pub fn new() -> Self {
        Self { bitfield: AtomicU64::default() }
    }
    // Create a bitfield with a single set bit at the start
    pub fn new_one() -> Self {
        Self { bitfield: AtomicU64::new(1) }
    }
    // Create a bitfield from the number literal
    pub fn from_num(n: u64) -> Self {
        Self { bitfield: AtomicU64::new(n) }
    }
    // Set the bitfield's bitfield.
    pub fn set_bitfield(&mut self, bitfield: u64) {
        self.bitfield = AtomicU64::new(bitfield);
    }
}

impl AtomicBitfieldU64 {
    // Add two bitfields together
    pub fn add(&self, other: &Self) -> Bitfield<T> {
        Self::from_num(self.bitfield | other.bitfield)
    }
    // Remove a bitfield from another bitfield
    pub fn remove(&self, other: &Self) -> Bitfield<T> {
        Self::from_num(!self.bitfield & other.bitfield)
    }
    // Check if a bitfield is contained within another bitfield
    pub fn contains(&self, other: &Self) -> bool {
        Self::empty(&Self::from_num(!self.bitfield & other.bitfield))
    }
    // Check if the bitfield is empty
    pub fn empty(&self) -> bool { self.bitfield == T::default() }
}
