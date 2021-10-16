use std::ops::Range;

use logos::{Lexer as LogosLexer, Logos};

#[derive(Debug)]
pub struct Token<'a> {
  pub value: &'a str,
  pub kind: TokenKind,
  pub range: Range<usize>,
}

pub struct Lexer<'a> {
  inner: LogosLexer<'a, TokenKind>,
  upper_bound: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      inner: TokenKind::lexer(source),
      upper_bound: source.len(),
    }
  }
  pub fn next_token(&mut self) -> Option<Token<'a>> {
    let kind = self.inner.next()?;
    let value = self.inner.slice();
    let range = self.inner.span();
    Some(Token { kind, value, range })
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;
  fn size_hint(&self) -> (usize, Option<usize>) {
    (0, Some(self.upper_bound))
  }
  fn next(&mut self) -> Option<Self::Item> {
    self.next_token()
  }
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
  #[regex(r"[ \t\n\f]+", logos::skip)]
  #[error]
  Error,

  #[regex("[a-zA-Z_]+")]
  Identifier,

  #[regex("[0-9]+", priority = 2)]
  Number,

  #[regex("([0-9]+([.][0-9]*)?|[.][0-9]+)")]
  Float,

  #[token("+")]
  Plus,
  #[token("-")]
  Minus,
  #[token("*")]
  Multiply,
  #[token("/")]
  Divide,
  #[token("^")]
  Power,

  #[token("(")]
  LeftParentheses,
  #[token(")")]
  RightParentheses,
}

#[cfg(test)]
mod tests {
  use crate::{Lexer, TokenKind};

  fn check(input: &str, kind: TokenKind) {
    let mut lexer = Lexer::new(input);
    assert_eq!(lexer.next().unwrap().kind, kind);
  }
  #[test]
  fn can_parse_number() {
    check("1", TokenKind::Number)
  }
  #[test]
  fn can_parse_id() {
    check("pi", TokenKind::Identifier)
  }
  #[test]
  fn can_parse_float() {
    check("1.23", TokenKind::Float)
  }
  #[test]
  fn can_parse_plus() {
    check("+", TokenKind::Plus)
  }
  #[test]
  fn can_parse_minus() {
    check("-", TokenKind::Minus)
  }
  #[test]
  fn can_parse_multiply() {
    check("*", TokenKind::Multiply)
  }
  #[test]
  fn can_parse_divide() {
    check("/", TokenKind::Divide)
  }
  #[test]
  fn can_parse_left_parentheses() {
    check("(", TokenKind::LeftParentheses)
  }
  #[test]
  fn can_parse_right_parentheses() {
    check(")", TokenKind::RightParentheses)
  }
}
