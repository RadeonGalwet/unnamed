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

  #[regex("[a-zA-Z_][a-zA-Z0-9_-]*")]
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
  #[token("==")]
  Equal,
  #[token("!=")]
  NotEqual,
  #[token(">")]
  Greater,
  #[token(">=")]
  GreaterEqual,
  #[token("<")]
  Less,
  #[token("<=")]
  LessEqual,
  #[token("||")]
  Or,
  #[token("&&")]
  And,
  #[token("!")]
  Negate,
  #[token("=")]
  Assignment,

  #[token("(")]
  LeftParentheses,
  #[token(")")]
  RightParentheses,
  #[token("{")]
  LeftCurly,
  #[token("}")]
  RightCurly,
  #[token("->")]
  Arrow,
  #[token(",")]
  Comma,
  #[token(":")]
  Colon,
  #[token(";")]
  SemiColon,

  #[token("function")]
  Function,
  #[token("return")]
  Return,
  #[token("if")]
  If,
  #[token("else")]
  Else,
  #[token("true")]
  True,
  #[token("false")]
  False,
  #[token("let")]
  Let,
  #[token("mut")]
  Mut,
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
  fn can_parse_assignment() {
    check("=", TokenKind::Assignment)
  }
  #[test]
  fn can_parse_equal() {
    check("==", TokenKind::Equal)
  }
  #[test]
  fn can_parse_not_equal() {
    check("!=", TokenKind::NotEqual)
  }
  #[test]
  fn can_parse_less() {
    check("<", TokenKind::Less)
  }
  #[test]
  fn can_parse_less_equal() {
    check("<=", TokenKind::LessEqual)
  }
  #[test]
  fn can_parse_greeter() {
    check(">", TokenKind::Greater)
  }
  #[test]
  fn can_parse_greeter_equal() {
    check(">=", TokenKind::GreaterEqual)
  }
  #[test]
  fn can_parse_and() {
    check("&&", TokenKind::And)
  }
  #[test]
  fn can_parse_or() {
    check("||", TokenKind::Or)
  }
  #[test]
  fn can_parse_left_parentheses() {
    check("(", TokenKind::LeftParentheses)
  }
  #[test]
  fn can_parse_right_parentheses() {
    check(")", TokenKind::RightParentheses)
  }
  #[test]
  fn can_parse_left_curly() {
    check("{", TokenKind::LeftCurly)
  }
  #[test]
  fn can_parse_right_curly() {
    check("}", TokenKind::RightCurly)
  }
  #[test]
  fn can_parse_arrow() {
    check("->", TokenKind::Arrow)
  }
  #[test]
  fn can_parse_comma() {
    check(",", TokenKind::Comma)
  }
  #[test]
  fn can_parse_colon() {
    check(":", TokenKind::Colon)
  }
  #[test]
  fn can_parse_semi_colon() {
    check(";", TokenKind::SemiColon)
  }
  #[test]
  fn can_parse_function_keyword() {
    check("function", TokenKind::Function)
  }
  #[test]
  fn can_parse_return_keyword() {
    check("return", TokenKind::Return)
  }
}
