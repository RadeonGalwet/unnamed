pub mod ast;
pub mod expression;

use std::iter::Peekable;

use crate::common::error::{Error, ErrorKind};
use crate::common::result::Result;
use crate::common::source::Source;
use crate::lexer::cursor::Cursor;
use crate::lexer::token::TokenKind;
use crate::lexer::{token::Token, Lexer};

pub struct Parser<'a, 'b> {
  lexer: Peekable<Lexer<'a>>,
  source: Source<'a>,
  cursor: &'b mut Cursor<'a>,
}

impl<'a, 'b> Parser<'a, 'b> {
  pub fn new(lexer: Peekable<Lexer<'a>>, source: Source<'a>, cursor: &'b mut Cursor<'a>) -> Self {
    Self {
      lexer,
      source,
      cursor,
    }
  }

  pub fn next(&mut self) -> Result<'a, Token<'a>> {
    let token = self.lexer.next();
    match token {
      Some(token) => token,
      None => Err(Error::new(
        ErrorKind::UnexpectedEndOfInput,
        self.source,
        self.cursor.span(),
      )),
    }
  }
  pub fn peek(&mut self) -> Result<'a, Token<'a>> {
    let token = self.lexer.peek();
    match token {
      Some(token) => *token,
      None => Err(Error::new(
        ErrorKind::UnexpectedEndOfInput,
        self.source,
        self.cursor.span(),
      )),
    }
  }
  pub fn consume(&mut self, token_kind: TokenKind) -> Result<'a, Token<'a>> {
    if self.peek()?.kind == token_kind {
      self.next()
    } else {
      Err(Error::new(
        ErrorKind::UnexpectedEndOfInput,
        self.source,
        self.cursor.span(),
      ))
    }
  }
}
