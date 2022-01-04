use crate::{common::span::Span, max, min};

use super::{CalculateSpan, Node, Spanned};

#[derive(Clone, Copy, Debug)]

pub enum InfixOperator {
  Plus,
  Minus,
  Multiply,
  Divide,
}

#[derive(Clone, Debug)]

pub enum Expression<'a> {
  Binary {
    operator: InfixOperator,
    lhs: Box<Node<'a>>,
    rhs: Box<Node<'a>>,
  },
  Call {
    name: Spanned<&'a str>,
    arguments: Vec<Node<'a>>,
  },
}

impl<'a> CalculateSpan for Expression<'a> {
  fn calculate_span(&self) -> Span<usize> {
    match self {
      Expression::Binary { lhs, rhs, .. } => Span::new(
        min!(lhs.calculate_span().start, rhs.calculate_span().start),
        max!(lhs.calculate_span().end, rhs.calculate_span().end),
      ),
      Expression::Call { name, arguments } => Span::new(
        name.span.start,
        arguments[arguments.len() - 1].calculate_span().end,
      ),
    }
  }
}
