#[macro_export]
macro_rules! impl_bitfield {
    ($data: ty) => {        
        impl Bitfield<$data>
        {
            /// Create a new empty bitfield
            pub fn new() -> Self {
                Self { inner: 0 }
            }
            /// Create a bitfield with a single set bit at the start
            pub fn new_one() -> Self {
                Self { inner: 1 }
            }
            /// Create a bitfield from the number literal
            pub fn from_num(n: $data) -> Self {
                Self { inner: n }
            }
            /// Increment the current bitfield (Shift to the left)
            pub fn increment(&mut self) {
                self.inner = self.inner << 1;
            }
            /// Get a reference to the simple bitfield's bitfield
            pub fn bitfield(&self) -> $data {
                self.inner
            }
            /// Set the bitfield's inner bitfield.
            pub fn set_inner(&mut self, bitfield: $data) {
                self.inner = bitfield;
            }
        }

        impl Bitfield<$data>
        {
            /// Add two bitfields together
            pub fn add(&self, other: &Self) -> Bitfield<$data> {
                Self::from_num(self.inner | other.inner)
            }
            /// Remove a bitfield from another bitfield
            pub fn remove(&self, other: &Self) -> Option<Bitfield<$data>> {
                if !self.contains(other) { return None; /* Self does not contain other, so we cannot remove it */ }
                Some (Self::from_num(self.inner & !other.inner))
            }
            /// Check if *self* contains some bits from *other*. It doesn't have to be all bits though
            pub fn contains(&self, other: &Self) -> bool {
                Self::empty(&Self::from_num(!self.inner & other.inner)) && (!Self::empty(self) && !Self::empty(other))
            }
            /// Check if the bitfield is empty
            pub fn empty(&self) -> bool {
                self.inner == 0
            }
            /// Read the bit at the specifed position
            pub fn read(&self, position: $data) -> bool {
                (self.inner >> position) % 1 == 1
            }
            /// Set the bit at the specified position
            pub fn write(&mut self, position: $data, bit: bool) {
                self.inner |= (bit as $data) << position;
            }
        }
    };
}
