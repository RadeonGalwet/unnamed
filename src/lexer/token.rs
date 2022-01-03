use crate::common::{
  error::{Error, ErrorKind},
  source::Source,
  span::Span,
  utils::get_utf8_slice,
};

use super::Result;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenKind {
  Identifier,
  Integer,
  Float,
  Plus,
  Minus,
  Multiply,
  Divide,
  Equal,
  Let,
  Semicolon,
  LeftRoundBracket,
  RightRoundBracket
}
#[derive(Clone, Copy, Debug)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub source: Source<'a>,
  pub span: Span<usize>,
}

impl<'a> Token<'a> {
  pub fn value(&self) -> Result<'a, &'a str> {
    get_utf8_slice(self.source.code, self.span.start, self.span.end)
      .ok_or_else(|| Error::new(ErrorKind::UnexpectedEndOfInput, self.source, self.span))
  }
}
