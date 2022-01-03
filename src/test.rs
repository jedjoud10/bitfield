#[cfg(test)]
pub mod test {
    use std::{sync::Arc, thread::JoinHandle};

    use crate::{Bitfield, AtomicBufferedBitfield};

    #[test]
    // Test the bitfield logic
    pub fn test() {
        let b1 = Bitfield::<u8>::from_num(10); // 1010
        let b2 = Bitfield::<u8>::from_num(11); // 1011
        assert!(b2.contains(&b1));

        let t1 = Bitfield::<u8>::from_num(156); // 1001 1100
        let t2 = Bitfield::<u8>::from_num(20); // 0001 0100
        assert!(t1.contains(&t2));
        assert!(t1.remove(&Bitfield::<u8>::from_num(156)).empty());
    }

    #[test]
    // Test the atomic buffered bitfield logic
    pub fn test_atomic() {
        let bitfield = AtomicBufferedBitfield::new();
        // This should be empty
        assert!(!bitfield.get(0));

        // Set the bit
        bitfield.set(0, true);
        bitfield.set(1, true);
        bitfield.set(2, true);
        bitfield.set(1, false);
        bitfield.set(65, false);
        
        bitfield.set(100_000_000, true);

        // This should be filled
        assert!(bitfield.get(0));
        assert!(!bitfield.get(1));
        assert!(!bitfield.get(65));
        assert!(bitfield.get(100_000_000));
    }

    #[test]
    pub fn test_atomicity() {
        let bitfield = Arc::new(AtomicBufferedBitfield::new());
        let thread_join_handles = (0..10).map(|_| {
            // Create a thread
            let bitfield = bitfield.clone();
            std::thread::spawn(move || {
                // Change the bitfield a ton of times
                for i in 0..1000 {
                    bitfield.set(i, i % 2 == 0);
                }
            })
        }).collect::<Vec<JoinHandle<()>>>();

        // Join up all the threads
        for x in thread_join_handles {
            x.join().unwrap();
        }

        // Test
        for i in 0..1000 {
            println!("Bit {}: {}", i, bitfield.get(i));
        }
    }
}
