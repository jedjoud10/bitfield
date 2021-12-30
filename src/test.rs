#[cfg(test)]
pub mod test {
    use crate::Bitfield;

    #[test]
    // Test the bitfield logic
    pub fn test() {
        let b1 = Bitfield::<u8>::from_num(10);
        let b2 = Bitfield::<u8>::from_num(11);
        println!("{}", b1);
        assert!(b2.contains(&b1));

        let t1 = Bitfield::<u8>::from_num(152); // 1001 1000
        let t2 = Bitfield::<u8>::from_num(20); // 0001 0100
        assert!(t1.contains(&t2));
        let t3 = Bitfield::<u8>::add(&t1, &t2); // 1001 1100
        assert!(t3.contains(&t2) && t3.contains(&t1));

        assert_eq!(t3.bitfield(), 156);
        assert!(t3.remove(&Bitfield::<u8>::from_num(156)).empty());
    }
}
