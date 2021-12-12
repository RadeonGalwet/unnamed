use std::fmt::{Display, self, Formatter};

use crate::{cursor::Cursor};

#[derive(Clone, Copy, Debug)]
pub struct Span {
  pub start: usize,
  pub end: usize,
}

impl Span {
  pub fn expand<'a, 'b>(&self, cursor: &'b mut Cursor<'a>) -> &'a str {
    &cursor.input[self.start..self.end]
  }
}

impl Display for Span {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
    write!(f, "{}..{}", self.start, self.end)
  }
}
