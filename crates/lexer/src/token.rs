use logos::{Logos, Source};

use crate::range::Range;

#[derive(Clone, Copy, Debug)]
pub struct Token<'a, T>
where
  T: Logos<'a>,
{
  pub value: &'a <T::Source as Source>::Slice,
  pub kind: T,
  pub range: Range<usize>,
}

#[derive(Clone, Copy, PartialEq, Debug, Logos)]
pub enum TokenKind {
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,

  #[regex("[0-9]+")]
  Integer,
  #[regex(r"[0-9]+\.[0-9]+")]
  Float,
  #[regex("[a-zA-Z$_][a-zA-Z0-9$_]*")]
  Identifier,

  #[token("+")]
  Plus,
  #[token("-")]
  Minus,
  #[token("*")]
  Multiply,
  #[token("/")]
  Divide,
  #[token(">")]
  Greater,
  #[token(">=")]
  GreaterOrEqual,
  #[token("<")]
  Less,
  #[token("<=")]
  LessOrEqual,
  #[token("==")]
  Equal,
  #[token("=")]
  Assignment,

  #[token("let")]
  Let,
  #[token("while")]
  While,
  #[token("if")]
  If,
  #[token("else")]
  Else,
  #[token("function")]
  Function,

  #[token("(")]
  LeftParentheses,
  #[token(")")]
  RightParentheses,
  #[token("{")]
  LeftCurlyBraces,
  #[token("}")]
  RightCurlyBraces,

}
