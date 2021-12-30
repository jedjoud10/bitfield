use std::fmt::{Debug, Display};

use crate::data::BitfieldData;

// A simple bitfield that contains a generic
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitfield<T>
where
    T: BitfieldData,
{
    bitfield: T,
}

impl<T> Debug for Bitfield<T>
where
    T: BitfieldData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitfield").field("bitfield", &self.bitfield).finish()
    }
}
impl<T> Display for Bitfield<T>
where
    T: BitfieldData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bitfield {:b}", self.bitfield))
    }
}

impl<T> Bitfield<T>
where
    T: BitfieldData,
{
    // Create a new empty bitfield
    pub fn new() -> Self {
        Self { bitfield: T::default() }
    }
    // Create a bitfield with a single set bit at the start
    pub fn new_one() -> Self {
        Self { bitfield: T::default_one() }
    }
    // Create a bitfield from the number literal
    pub fn from_num(n: T) -> Self {
        Self { bitfield: n }
    }
    // Increment the current bitfield (Shift to the left)
    pub fn increment(&mut self) {
        self.bitfield = self.bitfield << T::default_one();
    }
    // Get a reference to the simple bitfield's bitfield
    pub fn bitfield(&self) -> T {
        self.bitfield
    }
    // Set the bitfield's bitfield.
    pub fn set_bitfield(&mut self, bitfield: T) {
        self.bitfield = bitfield;
    }
}

impl<T> Bitfield<T>
where
    T: BitfieldData,
{
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
        !Self::empty(&Self::from_num(self.bitfield & !other.bitfield))
    }
    // Check if the bitfield is empty
    pub fn empty(&self) -> bool { self.bitfield == T::default() }
}
