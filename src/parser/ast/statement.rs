use crate::common::span::Span;

use super::{CalculateSpan, Node, Spanned};

#[derive(Clone, Debug)]
pub enum Statement<'a> {
  LetBinding {
    name: Spanned<&'a str>,
    value: Box<Node<'a>>,
  },
}

impl<'a> CalculateSpan for Statement<'a> {
  fn calculate_span(&self) -> Span<usize> {
    match self {
      Statement::LetBinding { name, value } => {
        Span::new(name.span.start, value.calculate_span().end)
      }
    }
  }
}
