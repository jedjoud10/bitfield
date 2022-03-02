use std::sync::{
    atomic::{AtomicU64, Ordering},
    RwLock,
};

/// An atomic sparse bitfield that can only be used to set bits manually.
/// We can set bits concurrently since it uses an AtomicU64 internally
#[derive(Default)]
pub struct AtomicSparseBitfield {
    /// The buffers and their corresponding padding
    buffer: RwLock<Vec<AtomicU64>>,
}

impl AtomicSparseBitfield {
    /// Create a new empty atomic sparse bitfield with a specified pre allocated chunks
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: RwLock::new(Vec::from_iter((0..(capacity)).map(|_| AtomicU64::new(0)))),
        }
    }
    /// Create a new atomic sparse bitfield using an array of bools
    #[inline(always)]
    pub fn from_bools(bools: &[bool]) -> Self {
        let len = ((bools.len() as u64) / (64_u64)) + 1;
        let bitfield = Self::with_capacity(len as usize);
        for (location, bool_val) in bools.iter().enumerate() {
            bitfield.set(location, *bool_val);
        }
        bitfield
    }
    /// Get a bit at a specific location
    #[inline(always)]
    pub fn get(&self, location: usize) -> bool {
        // Calculate some index stuff
        let block_pos = location / 64;
        let bit_pos = location % 64;
        let readable = self.buffer.read().unwrap();
        if let Some(atomic) = readable.get(block_pos) {
            // We have the block, so we can read the bit directly
            // Get the bit value
            let old_atomic_val = atomic.load(Ordering::Relaxed);
            ((old_atomic_val) >> bit_pos) % 2 == 1
        } else {
            // The block does not exist!
            false
        }
    }
    /// Set the bit at a specific location, if that location does not exist, we will expand the hashmap
    #[inline(always)]
    pub fn set(&self, location: usize, bit: bool) {
        // Calculate some index stuff
        let block_pos = location / 64;
        let bit_pos = location % 64;
        // Check if we even have the block stored inside the buffer
        let readable = self.buffer.read().unwrap();
        if let Some(atomic) = readable.get(block_pos) {
            // We have the block, so we can set the bit directly
            // Create the new value using the bit
            let bit_val = (1_u64) << bit_pos;
            // If we are setting the bit as "true", we must OR it, but if we are setting it as "false", we must AND it
            let old_atomic_val = atomic.load(Ordering::Relaxed);
            let new_atomic_val: u64 = if bit {
                // OR it
                old_atomic_val | bit_val
            } else {
                // AND the NOT
                old_atomic_val & !bit_val
            };
            // Set the atomic value
            atomic.store(new_atomic_val, Ordering::Relaxed);
            return;
        }
        drop(readable);

        // We do not have the block, we must insert it
        let mut writable = self.buffer.write().unwrap();
        // Create the new block
        let atomic = AtomicU64::new(if bit { (1_u64) << bit_pos } else { 0 });
        // Resize to fit, then add
        let len = (block_pos).checked_sub(writable.len());
        if let Some(len) = len {
            writable.extend((0..len).map(|_| AtomicU64::new(0)));
            writable.push(atomic);
        } else {
            // If it fails it means that the value was already allocated, but not set
            *writable.get_mut(block_pos).unwrap() = atomic;
        }
    }
    /// Clear all the bits in this sparse bitfield
    #[inline(always)]
    pub fn clear(&self) {
        // Loop through every block and set it's atomic value to 0
        let readable = self.buffer.read().unwrap();
        for atomic in readable.iter() {
            atomic.store(0, Ordering::Relaxed)
        }
    }
}
