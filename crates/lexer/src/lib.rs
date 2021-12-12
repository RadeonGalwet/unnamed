#![feature(decl_macro)]
use cursor::Cursor;
use error::{LexingError, LexingErrorKind};
use token::Token;

pub mod cursor;
pub mod error;
pub mod macros;
pub mod span;
pub mod token;

pub struct Lexer<'a> {
  cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      cursor: Cursor::new(input),
    }
  }
  pub fn skip_whitespaces(&mut self) -> Result<(), LexingError> {
    while self.cursor.peek()? == ' ' && !self.cursor.eof() {
      self.cursor.next()?;
    }
    self.cursor.clear_span();
    Ok(())
  }
  pub fn is_id_start(&mut self) -> Result<bool, LexingError> {
    let char = self.cursor.peek()?;
    Ok('a' <= char && char <= 'z' || 'A' <= char && char <= 'Z' || char == '_' || char == '$')
  }
  pub fn is_id(&mut self) -> Result<bool, LexingError> {
    Ok(self.is_id_start()? || self.is_number_start()?)
  }
  pub fn is_number_start(&mut self) -> Result<bool, LexingError> {
    let char = self.cursor.peek()?;

    Ok(('0'..='9').contains(&char))
  }
  pub fn is_number(&mut self) -> Result<bool, LexingError> {
    let char = self.cursor.peek()?;
    Ok(self.is_number_start()? || char == '.')
  }
  pub fn read_id(&mut self) -> Result<&'a str, LexingError> {
    self.test(|lexer| lexer.is_id_start())?;

    while !self.cursor.eof() && self.is_id()? {
      self.cursor.next()?;
    }
    let span = self.cursor.span();
    self.cursor.clear_span();
    Ok(span.expand(&mut self.cursor))
  }
  pub fn test<F>(&mut self, check: F) -> Result<(), LexingError>
  where
    F: Fn(&mut Self) -> Result<bool, LexingError>,
  {
    if check(self)? {
      self.cursor.next()?;
      Ok(())
    } else {
      Err(LexingError::new(
        LexingErrorKind::UnexpectedToken,
        self.cursor.span(),
      ))
    }
  }
  pub fn read_number(&mut self) -> Result<Token, LexingError> {
    self.test(|lexer| lexer.is_number_start())?;
    let mut is_float = false;
    let mut has_error = false; // Hack for full token read
    while !self.cursor.eof() && self.is_number()? {
      if self.cursor.peek()? == '.' {
        if is_float {
          has_error = true;
        }
        is_float = true
      }
      self.cursor.next()?;
    }
    if has_error {
      return Err(LexingError::new(
        LexingErrorKind::TooManyFloatingPoints,
        self.cursor.span(),
      ));
    }
    let span = self.cursor.span();
    self.cursor.clear_span();
    let slice = span.expand(&mut self.cursor);
    
    if is_float {
      Ok(Token::Float(slice))
    } else {
      Ok(Token::Integer(slice))
    }
  }
  pub fn read_single_char(&mut self) -> Result<Token, LexingError> {
    todo!()
  }
  pub fn next_token(&mut self) -> Result<Token, LexingError> {
    self.skip_whitespaces()?;

    if self.is_id_start()? {
      return Ok(Token::Identifier(self.read_id()?));
    }
    if self.is_number_start()? {
      return self.read_number();
    }
    let token = match self.cursor.next()? {
      '+' => Ok(Token::Plus),
      '-' => Ok(Token::Minus),
      '*' => Ok(Token::Multiply),
      '/' => Ok(Token::Divide),
      _ => Err(LexingError::new(
        LexingErrorKind::UnexpectedToken,
        self.cursor.span(),
      )),
    };
    self.cursor.clear_span();
    token
  }
}
