use super::{source::Source, span::Span};

#[derive(Clone, Copy, Debug)]
pub struct Error<'a> {
  pub kind: ErrorKind,
  pub source: Source<'a>,
  pub span: Span<usize>,
}

impl<'a> Error<'a> {
  pub fn new(kind: ErrorKind, source: Source<'a>, span: Span<usize>) -> Self {
    Self { kind, source, span }
  }
}
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  UnexpectedEndOfInput,
  UnexpectedToken,
}
