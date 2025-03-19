use super::BitConverter;
use crate::test::CompressorTests;
use proptest::proptest;

proptest! {
    #[test]
    fn test_from_into_bits_u8(items: Vec<u8>) {
        BitConverter::default().test_encode_decode(items).unwrap()
    }

    #[test]
    fn test_from_into_bits_u32(items: Vec<u32>) {
        BitConverter::default().test_encode_decode(items).unwrap()
    }
}
