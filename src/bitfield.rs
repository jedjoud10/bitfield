use crate::data::BitfieldData;

// A simple bitfield that contains a generic
pub struct SimpleBitfield<T>
where
    T: BitfieldData,
{
    bitfield: T,
}

impl<T> SimpleBitfield<T>
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
}

impl<T> SimpleBitfield<T>
where
    T: BitfieldData,
{
    // Add two bitfields together
    pub fn add(&self, other: &Self) -> SimpleBitfield<T> {
        SimpleBitfield {
            bitfield: other.bitfield | other.bitfield,
        }
    }
    // Remove a bitfield from another bitfield
    pub fn remove(&self, other: &Self) -> SimpleBitfield<T> {
        SimpleBitfield {
            bitfield: !self.bitfield & other.bitfield,
        }
    }
    // Check if a bitfield is contained within another bitfield
    pub fn contains(&self, other: &Self) -> bool {
        (self.bitfield & !other.bitfield) != T::default()
    }
}

// Types
pub type BitfieldU8 = SimpleBitfield<u8>;
pub type BitfieldU16 = SimpleBitfield<u16>;
pub type BitfieldU32 = SimpleBitfield<u32>;
pub type BitfieldU64 = SimpleBitfield<u64>;
pub type BitfieldU128 = SimpleBitfield<u128>;
