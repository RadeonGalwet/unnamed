use logos::{Lexer, Logos};
use range::Range;
use token::Token;

pub mod range;
pub mod token;

pub struct TokenStream<'a, T>
where
  T: Logos<'a>,
{
  pub lexer: Lexer<'a, T>,
}

impl<'a, T> Iterator for TokenStream<'a, T>
where
  T: Logos<'a>,
{
  type Item = Token<'a, T>;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.lexer.next()?;
    let value = self.lexer.slice();
    let range = Range::from(self.lexer.span());
    Some(Token { kind, value, range })
  }
}

impl<'a, T> TokenStream<'a, T>
where
  T: Logos<'a>,
{
  pub fn new(lexer: Lexer<'a, T>) -> Self {
    Self { lexer }
  }
}

#[cfg(test)]
mod tests {
  use logos::Logos;

  use crate::{token::TokenKind, TokenStream};

  fn check(input: &str, kind: TokenKind) {
    let mut lexer = TokenStream::new(TokenKind::lexer(input));
    assert_eq!(lexer.next().unwrap().kind, kind)
  }
  #[test]
  fn can_parse_integer() {
    check("1", TokenKind::Integer)
  }
  #[test]
  fn can_parse_integer_with_many_chars() {
    check("12", TokenKind::Integer)
  }
  #[test]
  fn can_parse_float() {
    check("1.1", TokenKind::Float)
  }
  #[test]
  fn can_parse_float_with_many_chars() {
    check("12.23", TokenKind::Float)
  }
  #[test]
  fn can_parse_identifier() {
    check("$abc_1", TokenKind::Identifier);
  }
  #[test]
  fn can_parse_plus() {
    check("+", TokenKind::Plus);
  }
  #[test]
  fn can_parse_minus() {
    check("-", TokenKind::Minus);
  }
  #[test]
  fn can_parse_multiply() {
    check("*", TokenKind::Multiply);
  }
  #[test]
  fn can_parse_divide() {
    check("/", TokenKind::Divide);
  }
  #[test]
  fn can_parse_greater() {
    check(">", TokenKind::Greater);
  }
  #[test]
  fn can_parse_greater_or_equal() {
    check(">=", TokenKind::GreaterOrEqual);
  }
  #[test]
  fn can_parse_less() {
    check("<", TokenKind::Less);
  }
  #[test]
  fn can_parse_less_or_equal() {
    check("<=", TokenKind::LessOrEqual);
  }
  #[test]
  fn can_parse_equal() {
    check("==", TokenKind::Equal);
  }
  #[test]
  fn can_parse_assignment() {
    check("=", TokenKind::Assignment);
  }
  #[test]
  fn can_parse_let() {
    check("let", TokenKind::Let);
  }
  #[test]
  fn can_parse_while() {
    check("while", TokenKind::While);
  }
  #[test]
  fn can_parse_if() {
    check("if", TokenKind::If);
  }
  #[test]
  fn can_parse_else() {
    check("else", TokenKind::Else);
  }
  #[test]
  fn can_parse_function() {
    check("function", TokenKind::Function);
  }
  #[test]
  fn can_parse_left_parentheses() {
    check("(", TokenKind::LeftParentheses);
  }
  #[test]
  fn can_parse_right_parentheses() {
    check(")", TokenKind::RightParentheses);
  }
  #[test]
  fn can_parse_left_curly_braces() {
    check("{", TokenKind::LeftCurlyBraces);
  }
  #[test]
  fn can_parse_right_curly_braces() {
    check("}", TokenKind::RightCurlyBraces);
  }
}
