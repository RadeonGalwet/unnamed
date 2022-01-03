pub mod cursor;
pub mod token;
#[macro_use]
pub mod r#macro;

use std::result;

use unicode_xid::UnicodeXID;

use crate::common::{
  error::{Error, ErrorKind},
  source::Source,
};

use self::{
  cursor::Cursor,
  r#macro::{product, single_product},
  token::Token,
};

// Compiler bug
#[allow(unused_imports)]
use self::r#macro::token;

type Result<'a, T> = result::Result<T, Error<'a>>;
#[derive(Clone, Debug)]
pub struct Lexer<'a> {
  pub cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(source: Source<'a>) -> Self {
    Self {
      cursor: Cursor::new(source),
    }
  }
  pub fn is_id_start(&mut self) -> Result<'a, bool> {
    Ok(UnicodeXID::is_xid_start(self.cursor.peek()?))
  }
  pub fn is_id_continue(&mut self) -> Result<'a, bool> {
    Ok(UnicodeXID::is_xid_continue(self.cursor.peek()?))
  }
  pub fn is_line_comment(&mut self) -> Result<'a, bool> {
    Ok(self.cursor.peek()? == '/' && self.cursor.lookup(1)? == '/')
  }
  pub fn is_block_comment(&mut self) -> Result<'a, bool> {
    Ok(self.cursor.peek()? == '/' && self.cursor.lookup(1)? == '*')
  }
  pub fn read_id(&mut self) -> Result<'a, ()> {
    self.cursor.next();
    while !self.cursor.eof() && self.is_id_continue()? {
      self.cursor.next();
    }
    Ok(())
  }
  pub fn skip(&mut self) -> Result<'a, ()> {
    while !self.cursor.eof() && self.cursor.peek()? == ' ' || self.cursor.peek()? == '\n' {
      self.cursor.next();
    }
    self.cursor.clear_span();
    Ok(())
  }
  pub fn skip_line_comment(&mut self) -> Result<'a, ()> {
    while !self.cursor.eof() && self.cursor.peek()? != '\n' {
      self.cursor.next();
    }
    self.skip()?;
    self.cursor.clear_span();
    Ok(())
  }
  pub fn is_number_start(&mut self) -> Result<'a, bool> {
    let char = self.cursor.peek()?;
    Ok(('0'..='9').contains(&char))
  }
  pub fn is_number_continue(&mut self) -> Result<'a, bool> {
    let char = self.cursor.peek()?;
    Ok(self.is_number_start()? || char == '.')
  }
  pub fn skip_block_comment(&mut self) -> Result<'a, ()> {
    while !self.cursor.eof() && self.cursor.peek()? != '*' && self.cursor.lookup(1)? != '/' {
      self.cursor.next();
    }
    self.cursor.next();
    self.cursor.next();
    self.cursor.clear_span();
    Ok(())
  }
  pub fn skip_comments(&mut self) -> Result<'a, ()> {
    if self.is_line_comment()? {
      self.skip_line_comment()?;
      self.skip()?;

    }
    if self.is_block_comment()? {
      self.cursor.next();
      self.cursor.next();
      self.skip_block_comment()?;
      self.skip()?;

    }
    if !self.cursor.eof() && self.is_block_comment()? || self.is_line_comment()? {
      self.skip_comments()?;
    }
    Ok(())
  }
  pub fn read_number(&mut self) -> Result<'a, Token<'a>> {
    let mut has_error = false;
    let mut is_float = false;
    while self.is_number_continue()? {
      if self.cursor.peek()? == '.' {
        if is_float {
          has_error = true;
        }
        is_float = true
      }
      self.cursor.next();
    }
    if has_error {
      return Err(Error::new(
        ErrorKind::UnexpectedToken,
        self.cursor.source,
        self.cursor.span(),
      ))
    }
    let token = if is_float {
      token!(self, Float)
    } else {
      token!(self, Integer)
    };
    self.cursor.clear_span();
    Ok(token)
  }
  pub fn single_char(&mut self) -> Result<'a, Token<'a>> {
    match self.cursor.peek()? {
      '+' => single_product!(self, Plus),
      '-' => single_product!(self, Minus),
      '*' => single_product!(self, Multiply),
      '/' => single_product!(self, Divide),
      _ => Err(Error::new(
        ErrorKind::UnexpectedToken,
        self.cursor.source,
        self.cursor.span(),
      )),
    }
  }
  pub fn next_token(&mut self) -> Result<'a, Token<'a>> {
    self.skip()?;
    self.skip_comments()?;
    if self.is_id_start()? {
      return product!(self, read_id, Identifier)
    };
    if self.is_number_start()? {
      return self.read_number()
    }
    self.single_char()
  }
}
