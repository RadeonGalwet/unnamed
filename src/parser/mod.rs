pub mod ast;
pub mod expression;
pub mod statement;

use std::iter::Peekable;

use crate::common::error::{Error, ErrorKind};
use crate::common::result::Result;
use crate::common::source::Source;
use crate::lexer::cursor::Cursor;
use crate::lexer::token::TokenKind;
use crate::lexer::{token::Token, Lexer};

use self::ast::{Block, Program};

pub struct Parser<'a, 'b> {
  lexer: Peekable<Lexer<'a>>,
  source: Source<'a>,
  cursor: &'b Cursor<'a>,
}

impl<'a, 'b> Parser<'a, 'b> {
  pub fn new(lexer: Peekable<Lexer<'a>>, source: Source<'a>, cursor: &'b Cursor<'a>) -> Self {
    Self {
      lexer,
      source,
      cursor,
    }
  }
  pub fn parse(&mut self) -> Result<'a, Program<'a>> {
    Ok(Program {
      block: Block(self.statements()?),
      source: self.source,
    })
  }
  pub fn next_token(&mut self) -> Result<'a, Token<'a>> {
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
      self.next_token()
    } else {
      Err(Error::new(
        ErrorKind::UnexpectedEndOfInput,
        self.source,
        self.cursor.span(),
      ))
    }
  }
  pub fn test(&mut self, kind: TokenKind) -> bool {
    if let Ok(token) = self.peek() {
      token.kind == kind
    } else {
      false
    }
  }
  pub fn test_and_next(&mut self, kind: TokenKind) -> Result<'a, bool> {
    if self.test(kind) {
      self.next_token()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }
  pub fn arguments<F, T>(&mut self, function: F, test: Option<TokenKind>) -> Result<'a, Vec<T>>
  where
    F: Fn(&mut Self) -> Result<'a, T>,
  {
    let mut args = vec![];
    loop {
      if let Some(test) = test {
        if self.test(test) {
          args.push(function(self)?);
        } else {
          break;
        }
      } else {
        args.push(function(self)?);
      }
      if !self.test_and_next(TokenKind::Comma)? {
        break;
      }
    }
    Ok(args)
  }
}
