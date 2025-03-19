use super::{Compressor, IteratorTransforms};
use std::fmt::Debug;

/// Testing methods for `Compressor` implementations.<br>
/// This is to make testing simpler and hopefully encourage it...
pub trait CompressorTests: Compressor + Clone
where
    Self::Item: Clone + PartialEq + Debug,
    Self::Data: Clone + PartialEq + Debug,
{
    /// Utility method to test encoding and then decoding an iterator.<br>
    /// This **should** be used for testing any `Compressor` implementation.
    fn test_encode_decode(self, items: Vec<Self::Item>) -> Result<(), Self::Error> {
        let encoded = items
            .clone()
            .into_iter()
            .apply(self.clone().encoder())
            .collect::<Result<Vec<_>, _>>()?;

        let decoded = dbg!(encoded)
            .into_iter()
            .apply(self.decoder())
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(items, decoded);
        Ok(())
    }
}

impl<C: Compressor> CompressorTests for C
where
    C: Clone,
    C::Item: Clone + PartialEq + Debug,
    C::Data: Clone + PartialEq + Debug,
{
}
