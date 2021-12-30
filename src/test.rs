pub mod test {
    use crate::BitfieldU8;

    #[test]
    // Test the bitfield logic
    pub fn test() {
        let b1 = BitfieldU8::from_num(10);
        let b2 = BitfieldU8::from_num(11);
        assert!(b2.contains(&b1));
    }
}
