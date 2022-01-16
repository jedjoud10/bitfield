#[cfg(test)]
pub mod test {
    use std::{sync::Arc, thread::JoinHandle};

    use crate::{AtomicSparseBitfield, Bitfield};

    #[test]
    // Test the bitfield logic
    pub fn test() {
        let b1 = Bitfield::<u8>::from_num(10); // 1010
        let b2 = Bitfield::<u8>::from_num(11); // 1011
        assert!(b2.contains(&b1));

        let t1 = Bitfield::<u8>::from_num(156); // 1001 1100
        let t2 = Bitfield::<u8>::from_num(20); // 0001 0100

        let c1 = Bitfield::<u8>::from_num(0); // 0000 0000
        let c2 = Bitfield::<u8>::from_num(0); // 0000 0000

        let y1 = Bitfield::<u8>::from_num(0); // 0000 0000
        let y2 = Bitfield::<u8>::from_num(1); // 0000 0001
        let y3 = Bitfield::<u8>::from_num(3); // 0000 0011
        assert!(t1.contains(&t2));
        assert!(t1.remove(&Bitfield::<u8>::from_num(156)).empty());
        assert!(!c1.contains(&c2));
        assert!(!y2.contains(&y1));
        assert!(!y1.contains(&y2));
        assert!(!y2.contains(&y3));
        assert!(y3.contains(&y2));
    }

    #[test]
    // Test the atomic buffered bitfield logic
    pub fn test_atomic() {
        let bitfield = AtomicSparseBitfield::new();
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
        let bitfield = Arc::new(AtomicSparseBitfield::new());
        let thread_join_handles = (0..10)
            .map(|_| {
                // Create a thread
                let bitfield = bitfield.clone();
                std::thread::spawn(move || {
                    // Change the bitfield a ton of times
                    for i in 0..1000 {
                        bitfield.set(i, i % 2 == 0);
                    }
                })
            })
            .collect::<Vec<JoinHandle<()>>>();

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
