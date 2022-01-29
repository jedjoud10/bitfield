use std::{fmt::{Debug, Display}, mem::size_of};

use crate::data::BitfieldData;

/// A simple bitfield that contains a generic
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitfield<T>
where
    T: BitfieldData,
{
    inner: T,
}

impl<T> Debug for Bitfield<T>
where
    T: BitfieldData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitfield").field("bitfield", &self.inner).finish()
    }
}
impl<T> Display for Bitfield<T>
where
    T: BitfieldData,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bitfield {:b}", self.inner))
    }
}

impl<T> Bitfield<T>
where
    T: BitfieldData,
{
    /// Create a new empty bitfield
    pub fn new() -> Self {
        Self { inner: T::default() }
    }
    /// Create a bitfield with a single set bit at the start
    pub fn new_one() -> Self {
        Self { inner: T::default_one() }
    }
    /// Create a bitfield from the number literal
    pub fn from_num(n: T) -> Self {
        Self { inner: n }
    }
    /// Increment the current bitfield (Shift to the left)
    pub fn increment(&mut self) {
        self.inner = self.inner << T::default_one();
    }
    /// Get a reference to the simple bitfield's bitfield
    pub fn bitfield(&self) -> T {
        self.inner
    }
    /// Set the bitfield's bitfield.
    pub fn set_bitfield(&mut self, bitfield: T) {
        self.inner = bitfield;
    }
}

impl<T> Bitfield<T>
where
    T: BitfieldData,
{
    /// Add two bitfields together
    pub fn add(&self, other: &Self) -> Bitfield<T> {
        Self::from_num(self.inner | other.inner)
    }
    /// Remove a bitfield from another bitfield
    pub fn remove(&self, other: &Self) -> Option<Bitfield<T>> {
        if !self.contains(other) { return None; /* Self does not contain other, so we cannot remove it */ }
        Some (Self::from_num(self.inner & !other.inner))
    }
    /// Check if *self* contains some bits from *other*. It doesn't have to be all bits though
    pub fn contains(&self, other: &Self) -> bool {
        Self::empty(&Self::from_num(!self.inner & other.inner)) && (!Self::empty(self) && !Self::empty(other))
    }
    /// Check if the bitfield is empty
    pub fn empty(&self) -> bool {
        self.inner == T::default()
    }
}
