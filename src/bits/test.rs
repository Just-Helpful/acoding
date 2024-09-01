use super::{FromBits, IntFromBits, IntIntoBits, IntoBits};
use crate::IteratorTransforms;
use proptest::proptest;
use std::fmt::Debug;

fn from_into_bits<D: IntFromBits + IntIntoBits + Debug>(items: Vec<D>) {
  let bits = items
    .clone()
    .into_iter()
    .apply(IntoBits::default())
    .collect::<Vec<_>>();

  let data = bits
    .into_iter()
    .apply(FromBits::default())
    .collect::<Vec<_>>();
  assert_eq!(items, data)
}

proptest! {
  #[test]
  fn test_from_into_bits_u8(items: Vec<u8>) {
    from_into_bits(items)
  }

  #[test]
  fn test_from_into_bits_u32(items: Vec<u32>) {
    from_into_bits(items)
  }
}

fn into_from_bits<D: IntFromBits + IntIntoBits + Debug>(items: Vec<bool>) {
  let data = items
    .clone()
    .into_iter()
    .apply(FromBits::default())
    .collect::<Vec<D>>();

  let bits = data
    .into_iter()
    .apply(IntoBits::default())
    .take(items.len())
    .collect::<Vec<_>>();
  assert_eq!(items, bits)
}

proptest! {
  #[test]
  fn test_into_from_bits_u8(items: Vec<bool>) {
    into_from_bits::<u8>(items)
  }

  #[test]
  fn test_into_from_bits_u32(items: Vec<bool>) {
    into_from_bits::<u32>(items)
  }
}
