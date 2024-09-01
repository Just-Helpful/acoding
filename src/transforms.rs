use std::iter::Iterator;

/// A transformation on iterators.
///
/// Importantly, this allows for iterators that output fewer items than<br>
/// the original iterators that the were applied to, which happens to<br>
/// be quite useful when compressing data.
pub trait Transform<In>: Sized {
  type Out;

  /// A next method that pulls items out of a prior iterator.
  fn next(&mut self, iter: &mut impl Iterator<Item = In>) -> Option<Self::Out>;

  /// We allow for a size hint, this is sometimes informative for simple<br>
  /// operations, such at bit -> byte conversions, where the size can<br>
  /// determined with simple operations on the underlying iterator.
  fn size_hint(&self, _iter: &impl Iterator) -> (usize, Option<usize>) {
    (0, None)
  }

  /// Combines this transform with another.
  ///
  /// The combined transform will apply `Self` first and then use its<br>
  /// output in the `transform` passed to this method.
  fn with<T: Transform<Self::Out>>(self, transform: T) -> (Self, T) {
    (self, transform)
  }
}

impl<In, T: Transform<In>> Transform<In> for &mut T {
  type Out = T::Out;
  fn next(&mut self, iter: &mut impl Iterator<Item = In>) -> Option<Self::Out> {
    T::next(self, iter)
  }
}

impl<I, T1: Transform<I>, T2: Transform<T1::Out>> Transform<I> for (T1, T2) {
  type Out = T2::Out;
  fn next(&mut self, iter: &mut impl Iterator<Item = I>) -> Option<Self::Out> {
    let mut iter = TransformIterator {
      source: iter,
      transform: &mut self.0,
    };
    self.1.next(&mut iter)
  }
}

/// Transform utility traits on iterators
pub trait IteratorTransforms: Iterator + Sized {
  /// Apply a transform to the iterator.
  /// This results in an iterator that can produce more or less items than the
  /// original iterator
  fn apply<T: Transform<Self::Item>>(self, transform: T) -> TransformIterator<Self, T> {
    TransformIterator {
      source: self,
      transform,
    }
  }
}

pub struct TransformIterator<S, T> {
  source: S,
  transform: T,
}

impl<S: Iterator, T: Transform<S::Item>> Iterator for TransformIterator<S, T> {
  type Item = T::Out;

  fn next(&mut self) -> Option<Self::Item> {
    self.transform.next(&mut self.source)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.transform.size_hint(&self.source)
  }
}

impl<I: Iterator> IteratorTransforms for I {}
