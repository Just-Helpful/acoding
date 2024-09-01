use crate::Compressor;
use std::convert::Infallible;

/// Generic types and constants for frequency and code values
mod arithmetic_codes {
  pub type Code = u32;
  pub const CODE_MAX: Code = (1 << (Code::BITS / 2 + 1)) - 1;
  pub const FREQ_MAX: Code = Code::MAX / (CODE_MAX + 1);
  pub const CODE_1_2: Code = CODE_MAX / 2 + 1;
  pub const CODE_1_4: Code = CODE_1_2 / 2;
  pub const CODE_3_4: Code = CODE_1_4 * 3;
}

mod frequency;
pub use frequency::{FreqTable, SymbolFreq};
mod encoder;
pub use encoder::ArithmeticEncoder;
mod decoder;
pub use decoder::ArithmeticDecoder;

/// The arithmetic coding compression algorithm.<br>
/// This is pretty much ripped directly from the excellent mark nelson [blog](https://marknelson.us/posts/2014/10/19/data-compression-with-arithmetic-coding.html),<br>
/// specifically the unsigned int implementation.
#[derive(Default, Clone, Copy)]
pub struct ArithmeticCompressor {
  frequencies: FreqTable,
}

impl Compressor for ArithmeticCompressor {
  type Error = Infallible;
  type Item = u8;
  type Data = bool;

  type Encoder = ArithmeticEncoder;
  type Decoder = ArithmeticDecoder;
}

#[cfg(test)]
mod test;
