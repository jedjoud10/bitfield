use std::fmt::{Binary, Debug};

// A bitfield data trait that can be implemented on un-signed integer types
pub trait BitfieldData:
    std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::BitXor<Output = Self>
    + std::ops::Not<Output = Self>
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + std::cmp::PartialEq
    + Copy
    + Sized
    + Default
    + Binary
    + Debug
{
    fn default_one() -> Self;
}
macro_rules! impl_bitfield_data {
    ($data: ty) => {
        impl BitfieldData for $data {
            fn default_one() -> Self {
                1
            }
        }
    };
}

// Implement the trait for the unsigned integers
impl_bitfield_data!(u8);
impl_bitfield_data!(u16);
impl_bitfield_data!(u32);
impl_bitfield_data!(u64);
impl_bitfield_data!(u128);
