use crate::{Compressor, IteratorTransforms};
use proptest::proptest;

use super::ArithmeticCompressor;

fn decode_encode(items: Vec<u8>) {
  let compressor = ArithmeticCompressor::default();

  let encoded = items
    .clone()
    .into_iter()
    .apply(compressor.encoder())
    .collect::<Result<Vec<_>, _>>()
    .expect("Arithmetic encoding won't fail on bits");

  let decoded = encoded
    .into_iter()
    .apply(compressor.decoder())
    .take(items.len())
    .collect::<Result<Vec<_>, _>>()
    .expect("Arithmetic decoding won't fail on bits");

  assert_eq!(items, decoded)
}

proptest! {
  #[test]
  fn decode_encode_test(items: Vec<u8>) {
    decode_encode(items)
  }
}

fn encode_decode(items: Vec<bool>) {
  let compressor = ArithmeticCompressor::default();

  let decoded = items
    .clone()
    .into_iter()
    .apply(compressor.decoder())
    .collect::<Result<Vec<_>, _>>()
    .expect("Arithmetic decoding won't fail on bits");

  let encoded = decoded
    .into_iter()
    .apply(compressor.encoder())
    .take(items.len())
    .collect::<Result<Vec<_>, _>>()
    .expect("Arithmetic encoding won't fail on bits");

  assert_eq!(items, encoded)
}

proptest! {
  #[test]
  fn encode_decode_test(items: Vec<bool>) {
    encode_decode(items)
  }
}
