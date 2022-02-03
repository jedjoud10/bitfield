use std::{fmt::{Debug, Display, Binary}};

use crate::impl_bitfield;

/// A simple bitfield that contains a generic
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bitfield<T>
{
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

// Implement the trait for the unsigned integers
impl_bitfield!(u8);
impl_bitfield!(u16);
impl_bitfield!(u32);
impl_bitfield!(u64);
impl_bitfield!(u128);
impl_bitfield!(usize);
impl_bitfield!(i8);
impl_bitfield!(i16);
impl_bitfield!(i32);
impl_bitfield!(i64);
impl_bitfield!(i128);
impl_bitfield!(isize);