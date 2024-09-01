use std::{
  marker::PhantomData,
  ops::{BitOrAssign, Shl},
};

use crate::Transform;

/// A transform that casts values to bytes
#[derive(Default)]
pub struct FromBits<D>(PhantomData<D>);

pub trait IntFromBits: From<bool> + Shl<u32, Output = Self> + BitOrAssign<Self> {
  const BITS: u32;
}
impl IntFromBits for u8 {
  const BITS: u32 = u8::BITS;
}
impl IntFromBits for u16 {
  const BITS: u32 = u16::BITS;
}
impl IntFromBits for u32 {
  const BITS: u32 = u32::BITS;
}

impl<D: IntFromBits> Transform<bool> for FromBits<D> {
  type Out = D;
  fn next(&mut self, iter: &mut impl Iterator<Item = bool>) -> Option<Self::Out> {
    let bit = iter.next()?;
    let mut byte = D::from(bit) << (D::BITS - 1);

    for i in (0..D::BITS - 1).rev() {
      let Some(bit) = iter.next() else { break };
      byte |= D::from(bit) << i
    }

    Some(byte)
  }

  fn size_hint(&self, iter: &impl Iterator) -> (usize, Option<usize>) {
    let (low, high) = iter.size_hint();
    (low / D::BITS as usize, high.map(|h| h / D::BITS as usize))
  }
}

impl<E, D: IntFromBits> Transform<Result<bool, E>> for FromBits<D> {
  type Out = Result<D, E>;
  fn next(&mut self, iter: &mut impl Iterator<Item = Result<bool, E>>) -> Option<Self::Out> {
    // take enough bits to construct a byte, propagating errors
    let bits: Vec<_> = match iter.take(D::BITS as usize).collect() {
      Ok(bits) => bits,
      Err(e) => return Some(Err(e)),
    };

    // construct a byte and wrap in `Ok`
    self.next(&mut bits.into_iter()).map(Ok)
  }

  fn size_hint(&self, iter: &impl Iterator) -> (usize, Option<usize>) {
    <Self as Transform<bool>>::size_hint(self, iter)
  }
}
