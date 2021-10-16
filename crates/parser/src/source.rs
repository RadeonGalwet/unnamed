use std::iter::Peekable;

use lexer::{Lexer, Token, TokenKind};

pub struct Source<'a> {
  pub(crate) stream: Peekable<Lexer<'a>>,
}

impl<'a> Source<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      stream: Lexer::new(source).peekable(),
    }
  }
  pub fn next_token(&mut self) -> Result<Token<'a>, String> {
    self
      .stream
      .next()
      .ok_or_else(|| "Unexpected end of input".to_string())
  }
  pub fn peek(&mut self) -> Result<&Token<'a>, String> {
    self
      .stream
      .peek()
      .ok_or_else(|| "Unexpected end of input".to_string())
  }
  pub fn test(&mut self, kind: TokenKind) -> bool {
    if let Ok(token) = self.peek() {
      token.kind == kind
    } else {
      false
    }
  }
  pub fn test_and_next(&mut self, kind: TokenKind) -> Result<bool, String> {
    if self.test(kind) {
      self.next_token()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }
  pub fn consume(&mut self, kind: TokenKind) -> Result<Token<'a>, String> {
    if self.test(kind) {
      self.next_token()
    } else {
      Err(format!("Expected {:?}, found {:?}", kind, self.peek()))
    }
  }
}
