//! # Compression Algorithms
//!
//! This crate provides traits for defining compression algorithms that operate
//! on iterators of values. There are two primary traits the crate defines:
//!
//! - `Transform`: A transform on iterators that can produce fewer or more items
//!   than the iterator the transformation is applied to.
//! - `Compressor`: An algorithm that provides both a method to encode an
//!   iterator and a method to decode the iterator produced.
mod arithmetic;
pub use arithmetic::ArithmeticEncoder;
mod bits;
pub use bits::{FromBits, IntFromBits, IntIntoBits, IntoBits};
mod transforms;
pub use transforms::{IteratorTransforms, Transform};
mod utils;
pub use utils::ResultWrap;
pub mod test;

/// A compression algorithm, capable of encoding and decoding streams of data.
///
/// ## Safety
///
/// This trait does **not** enforce that decoding an encoded iterator will<br>
/// reproduce the original iterator. It is up to the implementor to enforce<br>
/// this, though the `CompressorTest` trait can be used to verify.
pub unsafe trait Compressor: Sized {
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
