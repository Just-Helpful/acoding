mod from_bits;
use std::{convert::Infallible, marker::PhantomData};

pub use from_bits::{FromBits, IntFromBits};
mod into_bits;
pub use into_bits::{IntIntoBits, IntoBits};

use crate::{utils::ResultWrap, Compressor};

#[derive(Clone)]
pub struct BitConverter<D>(PhantomData<D>);

impl<D> Default for BitConverter<D> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<D: Default> From<BitConverter<D>> for (IntoBits<D>, ResultWrap<bool>) {
    fn from(_: BitConverter<D>) -> Self {
        Self::default()
    }
}

impl<D> From<BitConverter<D>> for (FromBits<D>, ResultWrap<D>) {
    fn from(_: BitConverter<D>) -> Self {
        Self::default()
    }
}

unsafe impl<D> Compressor for BitConverter<D>
where
    D: IntIntoBits + IntFromBits,
{
    type Error = Infallible;
    type Item = D;
    type Data = bool;

    type Encoder = (IntoBits<D>, ResultWrap<bool>);
    type Decoder = (FromBits<D>, ResultWrap<D>);
}

#[cfg(test)]
mod test;
