use super::{
  arithmetic_codes::{CODE_1_2, CODE_1_4, CODE_3_4, CODE_MAX},
  ArithmeticCompressor, FreqTable, SymbolFreq,
};
use crate::{IntoBits, Transform};
use std::convert::Infallible;

/// Arithmetic Decoding algorithm
pub struct ArithmeticDecoder {
  frequencies: FreqTable,
  range: (u32, u32),
  value: u32,
  pending: u32,
}

impl From<ArithmeticCompressor> for ArithmeticDecoder {
  fn from(value: ArithmeticCompressor) -> Self {
    Self {
      frequencies: value.frequencies,
      range: (0, CODE_MAX),
      value: 0,
      pending: 0,
    }
  }
}

impl From<ArithmeticCompressor> for (IntoBits<u8>, ArithmeticDecoder) {
  fn from(value: ArithmeticCompressor) -> Self {
    (Default::default(), value.into())
  }
}

impl ArithmeticDecoder {
  /// Fetches a bit from an iterator, marking `self` complete if it fails
  fn next_bit(&mut self, iter: &mut impl Iterator<Item = bool>) -> Option<u32> {
    if let Some(bit) = iter.next() {
      return Some(bit.into());
    }

    if self.pending > 0 {
      self.pending -= 1;
      return Some(0);
    }

    None
  }

  /// Shifts `self.range = (low, high)` and `self.value` left by one bit.<br>
  /// Filling `low` with `0`, `high` with `1` and `self.value` with `iter.next`.
  fn advance(&mut self, iter: &mut impl Iterator<Item = bool>) -> Option<()> {
    let (mut low, mut high) = self.range;
    (low, high) = (low << 1, (high << 1) | 1);
    self.range = (low & CODE_MAX, high & CODE_MAX);

    self.value <<= 1;
    self.value |= self.next_bit(iter)?;
    self.value &= CODE_MAX;
    Some(())
  }

  /// Fetches the initial data from the iterator or returns `None`
  fn fetch_value(iter: &mut impl Iterator<Item = bool>) -> u32 {
    let mut value = 0;
    for i in (0..CODE_MAX.trailing_ones()).rev() {
      let Some(bit) = iter.next() else { break };
      value |= u32::from(bit) << i;
    }
    value
  }
}

impl Transform<bool> for ArithmeticDecoder {
  type Out = Result<u8, Infallible>;
  fn next(&mut self, iter: &mut impl Iterator<Item = bool>) -> Option<Self::Out> {
    if self.pending == 0 {
      self.pending = CODE_MAX.trailing_ones();
      self.value = Self::fetch_value(iter);
    }

    // move low and high until low < CODE_1_4 and CODE_3_4 <= high
    // at which point we can decode the next character
    loop {
      let (low, high) = self.range;

      if (CODE_1_2 <= low) | (high < CODE_1_2) {
        self.advance(iter)?;
        continue;
      }

      if (CODE_1_4 <= low) & (high < CODE_3_4) {
        self.range = (low & !CODE_1_4, high | CODE_1_4);
        self.value -= CODE_1_4;
        self.advance(iter)?;
        continue;
      }

      break;
    }

    // decode next character
    let (low, high) = self.range;
    let width = high - low + 1;
    let len = self.frequencies.len();
    let i = ((self.value - low + 1) * len - 1) / width;

    let ((l, h), c) = self.frequencies.lookup(i)?;
    self.range = (
      low + (width * l) / len, //
      low + (width * h) / len - 1,
    );
    Some(Ok(c))
  }
}
