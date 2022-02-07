use std::{
    collections::HashMap,
};

/// A sparse bitfield that can only be used to set bits manually.
#[derive(Default)]
pub struct SparseBitfield {
    /// The buffers and their corresponding padding
    buffer: HashMap<u64, u128>,
}

impl SparseBitfield {
    /// Create a new empty atomic sparse bitfield with a specified pre allocated blocks
    pub fn with_capacity(num: u64) -> Self {
        Self {
            buffer: HashMap::from_iter((0..(num)).map(|x| (x, 0))),
        }
    }
    /// Create a new atomic sparse bitfield using an array of bools
    pub fn from_bools(bools: &[bool]) -> Self {
        let len = ((bools.len() as u64) / (128_u64)) + 1;
        let mut bitfield = Self::with_capacity(len);
        for (location, bool_val) in bools.iter().enumerate() {
            bitfield.set(location as u64, *bool_val);
        }
        bitfield
    }
    /// Get a bit at a specific location
    pub fn get(&self, location: u64) -> bool {
        // Calculate some index stuff
        let block_pos = location / 128;
        let bit_pos = location % 128;
        if let Some(block) = self.buffer.get(&block_pos) {
            // We have the block, so we can read the bit directly
            // Get the bit value
            ((*block) >> bit_pos) % 2 == 1
        } else {
            // The block does not exist!
            false
        }
    }
    /// Set the bit at a specific location, if that location does not exist, we will expand the hashmap
    pub fn set(&mut self, location: u64, bit: bool) {
        // Calculate some index stuff
        let block_pos = location / 128;
        let bit_pos = location % 128;
        // Check if we even have the block stored inside the buffer
        if let Some(block) = self.buffer.get_mut(&block_pos) {
            // We have the block, so we can set the bit directly
            // Create the new value using the bit
            let bit_val = (1_u128) << bit_pos;
            // If we are setting the bit as "true", we must OR it, but if we are setting it as "false", we must AND it
            *block = if bit {
                // OR it
                *block | bit_val
            } else {
                // AND the NOT
                *block & !bit_val
            };            
            return;
        }

        // We do not have the block, we must insert it
        // Create the new block
        let block = if bit { (1_u128) << bit_pos } else { 0 };
        self.buffer.insert(block_pos, block);
    }
    /// Clear all the bits in this sparse bitfield
    pub fn clear(&mut self) {
        // Loop through every block and set it's value to 0
        for block in self.buffer.iter_mut() { *block.1 = 0 }
    }
}
