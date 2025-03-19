use std::ops::{ShlAssign, Shr};

use crate::Transform;

/// A transform that casts values to bits, represented by `bool`s
pub struct IntoBits<D> {
  data: D,
  idx: u32,
}

impl<D: Default> Default for IntoBits<D> {
  fn default() -> Self {
    Self {
      data: D::default(),
      idx: 0,
    }
  }
}

pub trait IntIntoBits:
  PartialOrd + Default + ShlAssign<u32> + Shr<u32, Output = Self> + Copy
{
  const BITS: u32;
}
impl IntIntoBits for u8 {
  const BITS: u32 = u8::BITS;
}
impl IntIntoBits for u16 {
  const BITS: u32 = u16::BITS;
}
impl IntIntoBits for u32 {
  const BITS: u32 = u32::BITS;
}

impl<D: IntIntoBits> IntoBits<D> {
  fn next_bit(&mut self) -> Option<bool> {
    if self.idx == 0 {
      return None;
    }
    let bit = self.data >> (D::BITS - 1) > Default::default();
    self.data <<= 1;
    self.idx -= 1;
    Some(bit)
  }
}

impl<D: IntIntoBits> Transform<D> for IntoBits<D> {
  type Out = bool;
  fn next(&mut self, iter: &mut impl Iterator<Item = D>) -> Option<Self::Out> {
    self.next_bit().or_else(|| {
      self.data = iter.next()?;
      self.idx = D::BITS;
      self.next(iter)
    })
  }
}

impl<E, D: IntIntoBits> Transform<Result<D, E>> for IntoBits<D> {
  type Out = Result<bool, E>;
  fn next(&mut self, iter: &mut impl Iterator<Item = Result<D, E>>) -> Option<Self::Out> {
    self.next_bit().map(Ok).or_else(|| {
      let res = iter.next()?;
      self.data = match res {
        Ok(byte) => byte,
        Err(e) => return Some(Err(e)),
      };
      self.idx = D::BITS;
      self.next(iter)
    })
  }
}
