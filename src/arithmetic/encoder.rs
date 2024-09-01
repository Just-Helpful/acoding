use super::{
  arithmetic_codes::{CODE_1_2, CODE_1_4, CODE_3_4, CODE_MAX},
  frequency::{FreqTable, SymbolFreq},
  ArithmeticCompressor,
};
use crate::{FromBits, Transform};
use std::convert::Infallible;

/// Arithmetic Encoding algorithm
pub struct ArithmeticEncoder {
  frequencies: FreqTable,
  range: (u32, u32),
  pending_num: usize,
  pending_bit: Option<bool>,
}

impl From<ArithmeticCompressor> for ArithmeticEncoder {
  fn from(value: ArithmeticCompressor) -> Self {
    Self {
      frequencies: value.frequencies,
      range: (0, CODE_MAX),
      pending_num: 0,
      pending_bit: None,
    }
  }
}

impl From<ArithmeticCompressor> for (ArithmeticEncoder, FromBits<u8>) {
  fn from(value: ArithmeticCompressor) -> Self {
    (value.into(), Default::default())
  }
}

impl ArithmeticEncoder {
  /// Returns any bits still to be output,<br>
  /// resulting from fixing convergence on 0.5.
  fn pending_output(&mut self) -> Option<bool> {
    if self.pending_num == 0 {
      self.pending_bit = None;
    }
    let bit = self.pending_bit?;
    self.pending_num -= 1;
    Some(bit)
  }

  /// Shifts `self.range = (low, high)` left one bit,<br>
  /// filling `low` with `0` and `high` with `1`.<br>
  /// Masks off low and high with `MAX_CODE`.
  fn advance(&mut self) {
    let (mut low, mut high) = self.range;
    (low, high) = (low << 1, (high << 1) | 1);
    (low, high) = (low & CODE_MAX, high & CODE_MAX);
    self.range = (low, high);
  }
}

impl Transform<u8> for ArithmeticEncoder {
  type Out = Result<bool, Infallible>;
  fn next(&mut self, iter: &mut impl Iterator<Item = u8>) -> Option<Self::Out> {
    // if we have pending bits to output, output them
    if let Some(bit) = self.pending_output() {
      return Some(Ok(bit));
    }

    // both high and low are in one half:
    // expand range, add pending bits and output bit
    let (low, high) = self.range;
    if high < CODE_1_2 {
      self.advance();
      self.pending_bit = Some(true);
      return Some(Ok(false));
    }
    if low >= CODE_1_2 {
      self.advance();
      self.pending_bit = Some(false);
      return Some(Ok(true));
    }

    // low and high are likely to converge around 0.5:
    // increase the number of pending bits to output
    // and expand the range outwards
    if (low >= CODE_1_4) & (high < CODE_3_4) {
      self.pending_num += 1;
      self.range = (low & !CODE_1_4, high | CODE_1_4);
      self.advance();
    }
    // at this point, low < 0.25 && 0.75 <= high
    // therefore we can start decoding the next item
    else {
      // get next character or output pending bits
      let some_c = iter.next();
      let Some(c) = some_c else {
        return self.pending_output().map(Ok);
      };

      // decode frequencies for the next character
      let (l, h) = self.frequencies.range(c);
      let len = self.frequencies.len();

      // update range based on these frequencies
      let width = high - low + 1;
      self.range = (
        low + (width * l) / len, //
        low + (width * h) / len - 1,
      );
    }

    // we won't need to worry too much about this call
    // as it's basically the poster child for tail call optimisation
    self.next(iter)
  }
}
