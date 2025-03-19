use std::{convert::Infallible, marker::PhantomData};

use crate::Transform;

pub struct ResultWrap<I>(PhantomData<I>);

impl<I> Default for ResultWrap<I> {
  fn default() -> Self {
    Self(PhantomData)
  }
}

impl<I> Transform<I> for ResultWrap<I> {
  type Out = Result<I, Infallible>;
  fn next(&mut self, iter: &mut impl Iterator<Item = I>) -> Option<Self::Out> {
    let item = iter.next()?;
    Some(Ok(item))
  }
}
