use std::{error, fmt::Display};

use super::{source::Source, span::Span};

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
  UnexpectedEndOfInput,
  UnexpectedToken,
  TooManyFloatingPoints
}

#[derive(Debug, Clone, Copy)]
pub struct Error<'a> {
  pub kind: ErrorKind,
  pub span: Span<'a>,
  pub source: Source<'a>,
}

impl<'a> Error<'a> {
  pub fn new(kind: ErrorKind, span: Span<'a>, source: Source<'a>) -> Self {
    Self { kind, span, source }
  }
}
pub struct File<'a> {
  pub path: &'a str,
  pub source: &'a str,
}
impl<'a> error::Error for Error<'a> {}
impl<'a> Display for Error<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "test")
  }
}
