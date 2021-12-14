use std::iter::Peekable;

use lexer::Lexer;

pub struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      lexer: Lexer::new(input).peekable(),
    }
  }
  pub fn parse(&mut self) {}
}
