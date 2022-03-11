use getset::*;
use std::fmt::{Binary, Debug, Display};

/// A simple bitfield that contains a generic
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Getters, Setters, MutGetters, CopyGetters)]
pub struct Bitfield<T> {
    /// The inner value for this bitfield
    #[getset(get = "pub")]
    inner: T,
}

impl<T> Debug for Bitfield<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitfield").field("bitfield", &self.inner).finish()
    }
}
impl<T> Display for Bitfield<T>
where
    T: Display + Binary,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bitfield {:b}", self.inner))
    }
}

impl<T: num::Integer + Copy> Bitfield<T> {
    /// Create a new empty bitfield
    #[inline(always)]
    pub fn new() -> Self {
        Self { inner: T::zero() }
    }
    /// Create a bitfield with a single set bit at the start
    #[inline(always)]
    pub fn new_one() -> Self {
        Self { inner: T::one() }
    }
    /// Create a bitfield from the number literal
    #[inline(always)]
    pub fn from_num(n: T) -> Self {
        Self { inner: n }
    }
    /// Increment the current bitfield (Shift to the left)
    #[inline(always)]
    pub fn increment(&mut self)
    where
        T: std::ops::Shl<Output = T>,
    {
        self.inner = self.inner << T::one();
    }
    /// Decrement the current bitfield (Shift to the right)
    #[inline(always)]
    pub fn decrement(&mut self)
    where
        T: std::ops::Shr<Output = T>,
    {
        self.inner = self.inner >> T::one();
    }
}

impl<T: num::Integer + Copy> Bitfield<T> {
    /// Add two bitfields together
    #[inline(always)]
    pub fn add(&self, other: &Self) -> Bitfield<T>
    where
        T: std::ops::BitOr<Output = T>,
    {
        Self::from_num(self.inner | other.inner)
    }
    /// Remove a bitfield from another bitfield
    #[inline(always)]
    pub fn remove(&self, other: &Self) -> Option<Bitfield<T>>
    where
        T: std::ops::Not<Output = T> + std::ops::BitAnd<Output = T>,
    {
        if !self.contains(other) {
            return None; /* Self does not contain other, so we cannot remove it */
        }
        Some(Self::from_num(self.inner & !other.inner))
    }
    /// Check if *self* contains all the required bits from *other*.
    #[inline(always)]
    pub fn contains(&self, other: &Self) -> bool
    where
        T: std::ops::Not<Output = T> + std::ops::BitAnd<Output = T>,
    {
        Self::empty(&Self::from_num(!self.inner & other.inner)) && (!Self::empty(self) && !Self::empty(other))
    }
    /// Check if the bitfield is empty
    #[inline(always)]
    pub fn empty(&self) -> bool {
        self.inner == T::zero()
    }
    /// Read the bit at the specifed position
    #[inline(always)]
    pub fn read(&self, position: T) -> bool
    where
        T: std::ops::Shr<Output = T> + std::ops::Rem<Output = T>,
    {
        (self.inner >> position) % T::one() == T::one()
    }
    /// Set the bit at the specified position
    #[inline(always)]
    pub fn write(&mut self, position: T, bit: bool)
    where
        T: std::ops::Shl<Output = T> + std::ops::BitOrAssign,
    {
        self.inner |= (if bit { T::one() } else { T::zero() }) << position;
    }
}
