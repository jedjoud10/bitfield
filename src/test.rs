#[cfg(test)]
pub mod test {
    use crate::Bitfield;

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
}
