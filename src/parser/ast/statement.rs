use crate::common::span::Span;

use super::Node;

#[derive(Clone, Debug)]
pub enum Statement<'a> {
  LetBinding {
    name: &'a str,
    value: Box<Node<'a>>,
    span: Span<usize>,
  },
}
