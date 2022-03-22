use getset::*;
use std::{
    fmt::{Binary, Debug, Display},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign},
};

pub trait Bit: BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Sized + Copy + num::Integer {}
impl<T> Bit for T where T: BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Sized + Copy + num::Integer {}
/// A simple bitfield that contains a generic
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Getters, Setters, MutGetters, CopyGetters)]
pub struct Bitfield<T: Bit> {
    /// The inner value for this bitfield
    #[getset(get = "pub")]
    inner: T,
}

impl<T> Debug for Bitfield<T>
where
    T: Debug + Bit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Bitfield").field("bitfield", &self.inner).finish()
    }
}
impl<T> Display for Bitfield<T>
where
    T: Display + Binary + Bit,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bitfield {:b}", self.inner))
    }
}

impl<T: Bit> From<T> for Bitfield<T> {
    fn from(t: T) -> Self {
        Self { inner: t }
    }
}

impl<T: num::Integer + Copy + Bit> Bitfield<T> {
    /// Zero
    #[inline(always)]
    pub fn zero() -> Self {
        Self { inner: T::zero() }
    }
    /// One
    #[inline(always)]
    pub fn one() -> Self {
        Self { inner: T::one() }
    }
    /// Create a new empty bitfield
    #[inline(always)]
    pub fn new() -> Self {
        Self { inner: T::zero() }
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

impl<T: Bit> Bitfield<T> {
    /// Add two bitfields together
    #[inline(always)]
    pub fn add(&self, other: &Self) -> Bitfield<T>
    where
        T: std::ops::BitOr<Output = T>,
    {
        Self::from(self.inner | other.inner)
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
        Some(Self::from(self.inner & !other.inner))
    }
    /// Check if *self* contains all the required bits from *other*.
    #[inline(always)]
    pub fn contains(&self, other: &Self) -> bool
    where
        T: std::ops::Not<Output = T> + std::ops::BitAnd<Output = T>,
    {
        Self::empty(&Self::from(!self.inner & other.inner)) && (!Self::empty(self) && !Self::empty(other))
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

impl<T: Bit> BitOr for Bitfield<T> {
    type Output = Bitfield<T>;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitfield::from(self.inner | rhs.inner)
    }
}
impl<T: Bit> BitAnd for Bitfield<T> {
    type Output = Bitfield<T>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitfield::from(self.inner & rhs.inner)
    }
}
impl<T: Bit> BitXor for Bitfield<T> {
    type Output = Bitfield<T>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitfield::from(self.inner ^ rhs.inner)
    }
}
impl<T: Bit> BitOrAssign for Bitfield<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}
impl<T: Bit> BitAndAssign for Bitfield<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}
impl<T: Bit> BitXorAssign for Bitfield<T> {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}
