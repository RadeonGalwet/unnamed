use crate::{common::span::Span, max, min};

use super::Node;

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
}

impl<'a> Expression<'a> {
  pub fn calculate_span(&self) -> Span<usize> {
    match self {
      Expression::Binary { lhs, rhs, .. } => Span {
        start: min!(lhs.calculate_span().start, rhs.calculate_span().start),
        end: max!(lhs.calculate_span().end, rhs.calculate_span().end),
      },
    }
  }
}
