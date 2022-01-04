use std::fmt::Debug;

use crate::common::{source::Source, span::Span};

use self::{expression::Expression, statement::Statement};

pub mod expression;
pub mod statement;

pub trait CalculateSpan {
  fn calculate_span(&self) -> Span<usize>;
}

#[derive(Clone, Debug)]
pub enum Node<'a> {
  Integer(Spanned<&'a str>),
  Float(Spanned<&'a str>),
  Identifier(Spanned<&'a str>),
  Expression(Expression<'a>),
  Statement(Statement<'a>),
}

impl<'a> CalculateSpan for Node<'a> {
  fn calculate_span(&self) -> Span<usize> {
    match self {
      Node::Integer(value) => value.span,
      Node::Float(value) => value.span,
      Node::Identifier(value) => value.span,
      Node::Expression(expression) => expression.calculate_span(),
      Node::Statement(statement) => statement.calculate_span(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Block<'a>(pub Vec<Node<'a>>);
#[derive(Clone, Debug)]
pub struct Program<'a> {
  pub block: Block<'a>,
  pub source: Source<'a>,
}

#[derive(Clone, Debug)]
pub struct Spanned<T: Clone + Debug> {
  pub value: T,
  pub span: Span<usize>,
}

impl<T: Clone + Debug> Spanned<T> {
  pub fn new(value: T, span: Span<usize>) -> Self {
    Self { value, span }
  }
}
