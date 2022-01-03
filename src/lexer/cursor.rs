use crate::common::{
  error::{Error, ErrorKind},
  source::Source,
  span::Span,
  utils::get_utf8_slice,
};

use super::Result;

#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
  pub source: Source<'a>,
  pub start: usize,
  pub end: usize,
}

impl<'a> Cursor<'a> {
  pub fn new(source: Source<'a>) -> Self {
    Self {
      source,
      start: 0,
      end: 0,
    }
  }
  pub fn peek(&self) -> Result<'a, char> {
    get_utf8_slice(self.source.code, self.end, self.end + 1)
      .ok_or_else(|| Error::new(ErrorKind::UnexpectedEndOfInput, self.source, self.span()))?
      .chars()
      .next()
      .ok_or_else(|| Error::new(ErrorKind::UnexpectedEndOfInput, self.source, self.span()))
  }
  pub fn next(&mut self) {
    self.end += 1;
  }
  pub fn lookup(&mut self, lookup: usize) -> Result<'a, char> {
    get_utf8_slice(self.source.code, self.end + lookup, self.end + (lookup + 1))
    .ok_or_else(|| Error::new(ErrorKind::UnexpectedEndOfInput, self.source, self.span()))?
    .chars()
    .next()
    .ok_or_else(|| Error::new(ErrorKind::UnexpectedEndOfInput, self.source, self.span()))
  }
  pub fn span(&self) -> Span<usize> {
    Span {
      start: self.start,
      end: self.end,
    }
  }
  pub fn clear_span(&mut self) {
    self.start = self.end;
  }
  pub fn consume(&mut self, char: char) -> Result<'a, ()> {
    if self.peek()? == char {
      self.next();
      Ok(())
    } else {
      Err(Error::new(ErrorKind::UnexpectedToken, self.source, self.span()))
    }
  }
  #[inline]
  pub fn eof(&self) -> bool {
    self.end > self.source.code.chars().count() - 1
  }
}
