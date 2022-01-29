use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        RwLock,
    },
};

/// An atomic sparse bitfield that can only be used to set bits manually.
/// We can set bits concurrently since it uses an AtomicU64 internally
#[derive(Default)]
pub struct AtomicSparseBitfield {
    /// The buffers and their corresponding padding
    buffer: RwLock<HashMap<u64, AtomicU64>>,
}
impl AtomicSparseBitfield {
    /// Create a new empty atomic sparse bitfield with a specified pre allocated chunks
    pub fn new(num: u64) -> Self {
        Self {
            buffer: RwLock::new(HashMap::from_iter((0..(num)).map(|x| (x, AtomicU64::new(0))))),
        }
    }
    /// Create a new atomic sparse bitfield using an array of bools
    pub fn from_bools(bools: &[bool]) -> Self {
        let len = (bools.len() as u64).div_ceil(64_u64);
        let bitfield = Self::new(len);
        for (location, bool_val) in bools.iter().enumerate() {
            bitfield.set(location as u64, *bool_val);
        }
        bitfield
    }
    /// Get a bit at a specific location
    pub fn get(&self, location: u64) -> bool {
        // Calculate some index stuff
        let block_pos = location / 64;
        let bit_pos = location % 64;
        let readable = self.buffer.read().unwrap();
        if let Some(atomic) = readable.get(&block_pos) {
            // We have the block, so we can read the bit directly
            // Get the bit value
            let old_atomic_val = atomic.load(Ordering::Relaxed);
            //println!("Get old {:b}", old_atomic_val);

            //println!("Get shifted {:b}", (old_atomic_val) >> bit_pos);
            ((old_atomic_val) >> bit_pos) % 2 == 1
        } else {
            // The block does not exist!
            false
        }
    }
    /// Set the bit at a specific location, if that location does not exist, we will expand the hashmap
    pub fn set(&self, location: u64, bit: bool) {
        // Calculate some index stuff
        let block_pos = location / 64;
        let bit_pos = location % 64;
        // Check if we even have the block stored inside the buffer
        let readable = self.buffer.read().unwrap();
        if let Some(atomic) = readable.get(&block_pos) {
            // We have the block, so we can set the bit directly
            // Create the new value using the bit
            let bit_val = (1_u64) << bit_pos;
            // If we are setting the bit as "true", we must OR it, but if we are setting it as "false", we must AND it
            let old_atomic_val = atomic.load(Ordering::Relaxed);
            //println!("Old {:b}", old_atomic_val);
            let new_atomic_val: u64 = if bit {
                // OR it
                old_atomic_val | bit_val
            } else {
                // AND the NOT
                old_atomic_val & !bit_val
            };
            // Set the atomic value
            //println!("New {:b}", new_atomic_val);
            atomic.store(new_atomic_val, Ordering::Relaxed);
            return;
        }
        drop(readable);

        // We do not have the block, we must insert it
        let mut writable = self.buffer.write().unwrap();
        // Create the new block
        //println!("Insert {:b}", bit_val);
        let atomic = AtomicU64::new(if bit { (1_u64) << bit_pos } else { 0 });
        writable.insert(block_pos, atomic);
    }
}
