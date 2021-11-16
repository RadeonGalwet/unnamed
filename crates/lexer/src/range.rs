use core::fmt;
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Clone, Copy, Debug)]
pub struct Range<T: Clone + Copy> {
  pub start: T,
  pub end: T
}
impl<T: Display + Clone + Copy> Display for Range<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
      write!(f, "{}..{}", self.start, self.end)
    }
}

impl<T: Clone + Copy> From<ops::Range<T>> for Range<T> {
    fn from(range: ops::Range<T>) -> Self {
      Self {
        start: range.start,
        end: range.end
      }
    }
}