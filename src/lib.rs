mod arithmetic;
pub use arithmetic::ArithmeticEncoder;
mod bits;
pub use bits::{FromBits, IntFromBits, IntIntoBits, IntoBits};
mod transforms;
pub use transforms::{IteratorTransforms, Transform};

/// A compression algorithm, capable of encoding and decoding streams of data.
pub trait Compressor: Sized {
  /// Errors that occur during encoding and decoding data
  type Error;
  /// Uncompressed items given as inputs to the encoding algorithm
  type Item;
  /// Compressed data produced by the encoding algorithm
  type Data;

  /// The encoding algorithm.
  ///
  /// Should be constructible from `self`, to allow common data to be used<br>
  /// in the definition of both encoders and decoders
  type Encoder: Transform<Self::Item, Out = Result<Self::Data, Self::Error>> + From<Self>;
  /// The decoding algorithm.
  ///
  /// Should be constructible from `self`, to allow common data to be used<br>
  /// in the definition of both encoders and decoders
  type Decoder: Transform<Self::Data, Out = Result<Self::Item, Self::Error>> + From<Self>;

  /// Utility method for the encoder type
  fn encoder(self) -> Self::Encoder {
    self.into()
  }
  /// Utility method for the encoder type
  fn decoder(self) -> Self::Decoder {
    self.into()
  }
}
