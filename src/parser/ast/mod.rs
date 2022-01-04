use crate::common::{source::Source, span::Span};

use self::{expression::Expression, statement::Statement};

pub mod expression;
pub mod statement;

#[derive(Clone, Debug)]
pub enum Node<'a> {
  Integer { value: &'a str, span: Span<usize> },
  Float { value: &'a str, span: Span<usize> },
  String { value: &'a str, span: Span<usize> },
  Expression(Expression<'a>),
  Statement(Statement<'a>),
}

impl<'a> Node<'a> {
  pub fn calculate_span(&self) -> Span<usize> {
    match self {
      Node::Integer { span, .. } => *span,
      Node::Float { span, .. } => *span,
      Node::String { span, .. } => *span,
      Node::Expression(expression) => expression.calculate_span(),
      Node::Statement(_) => todo!(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct Block<'a>(Vec<Node<'a>>);
#[derive(Clone, Debug)]
pub struct Program<'a> {
  pub block: Block<'a>,
  pub source: Source<'a>,
}
