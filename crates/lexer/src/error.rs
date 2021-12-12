use std::{
  error::Error,
  fmt::{self, Display, Formatter},
};

use crate::span::Span;

#[derive(Clone, Copy, Debug)]
pub struct LexingError {
  pub kind: LexingErrorKind,
  pub span: Span,
}

impl LexingError {
  pub fn new(kind: LexingErrorKind, span: Span) -> Self {
    Self { kind, span }
  }
}
#[derive(Clone, Copy, Debug)]
pub enum LexingErrorKind {
  UnexpectedEndOfInput,
  UnexpectedToken,
  TooManyFloatingPoints,
}

impl Error for LexingError {}
impl Display for LexingError {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self.kind {
      LexingErrorKind::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
      LexingErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
      LexingErrorKind::TooManyFloatingPoints => {
        write!(f, "Invalid number of floating points in the number literal")
      }
    }
  }
}
