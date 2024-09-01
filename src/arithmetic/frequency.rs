use super::arithmetic_codes::{Code, FREQ_MAX};
use std::array;

pub trait SymbolFreq<C> {
  /// The sum of frequencies in the table
  fn len(&self) -> Code;
  /// The frequency range for `item`
  fn range(&self, item: C) -> (Code, Code);
  /// Searches for the item with a frequency range that contains `i`
  fn lookup(&self, i: Code) -> Option<((Code, Code), C)>;
}

#[derive(Clone, Copy)]
pub struct FreqTable([Code; 256]);

impl Default for FreqTable {
  fn default() -> Self {
    Self(array::from_fn(|i| i as Code + 1))
  }
}

impl SymbolFreq<u8> for FreqTable {
  #[inline]
  fn len(&self) -> Code {
    self.0[255]
  }

  #[inline]
  fn range(&self, item: u8) -> (Code, Code) {
    if item == 0 {
      return (0, self.0[0]);
    }
    let item = item as usize;
    (self.0[item - 1], self.0[item])
  }

  #[inline]
  fn lookup(&self, i: Code) -> Option<((Code, Code), u8)> {
    let pos = self.0.partition_point(|v| v <= &i);
    if pos == 256 {
      None
    } else {
      let pos = pos as u8;
      Some((self.range(pos), pos))
    }
  }
}

impl FreqTable {
  /// Adds `value` to the frequency of a single `item`
  pub fn add(&mut self, item: u8, value: Code) -> Option<()> {
    if self.0[255] == FREQ_MAX {
      return None;
    }
    for j in (item as usize)..256 {
      self.0[j] += value;
    }
    Some(())
  }

  /// Adds all the frequencies in `updates` to `self`
  pub fn update(&mut self, mut updates: [Code; 256]) -> Option<()> {
    // accumulate updates prior to adding
    let mut sum = 0;
    for update in &mut updates {
      sum += *update;
      *update = sum;
    }

    // equal to `updates[255] + self.0[255] < FREQ_MAX` but won't
    // overflow / underflow as `self.0[255] < FREQ_MAX` by invariants
    (updates[255] < FREQ_MAX - self.0[255]).then(|| {
      for (freq, update) in self.0.iter_mut().zip(updates) {
        *freq += update
      }
    })
  }
}
