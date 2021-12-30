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
    }
}
