use std::default;

use crate::{test::CompressorTests, IntoBits, IteratorTransforms, Transform};
use proptest::proptest;

use super::ArithmeticCompressor;

proptest! {
  #[test]
  fn test_encode_decode(items: Vec<u8>) {
    ArithmeticCompressor::default().test_encode_decode(items).unwrap()
  }
}
