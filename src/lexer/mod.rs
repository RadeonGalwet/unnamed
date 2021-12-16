use cursor::Cursor;
use token::Token;

use crate::common::{
  error::{Error, ErrorKind},
  source::Source,
};

pub mod cursor;
pub mod iterator;
pub mod token;

pub struct Lexer<'a> {
  pub cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: Source<'a>) -> Self {
    Self {
      cursor: Cursor::new(input),
    }
  }
  pub fn skip(&mut self) -> Result<(), Error<'a>> {
    while !self.cursor.eof() && (self.cursor.peek() == '\n' || self.cursor.peek() == ' ') {
      self.cursor.next_char();
    }
    self.cursor.clear_span();
    Ok(())
  }
  pub fn is_id_start(&mut self) -> Result<bool, Error<'a>> {
    let char = self.cursor.peek();
    Ok(('a'..='z').contains(&char) || ('A'..='Z').contains(&char) || char == '_' || char == '$')
  }
  pub fn is_id(&mut self) -> Result<bool, Error<'a>> {
    Ok(self.is_id_start()? || self.is_number_start()?)
  }
  pub fn is_number_start(&mut self) -> Result<bool, Error<'a>> {
    let char = self.cursor.peek();

    Ok(('0'..='9').contains(&char))
  }
  pub fn is_number(&mut self) -> Result<bool, Error<'a>> {
    let char = self.cursor.peek();
    Ok(self.is_number_start()? || char == '.')
  }
  pub fn read_id(&mut self) -> Result<&'a str, Error<'a>> {
    self.test(|lexer| lexer.is_id_start())?;

    while !self.cursor.eof() && self.is_id()? {
      self.cursor.next_char();
    }
    let span = self.cursor.span();
    self.cursor.clear_span();
    Ok(span.expand())
  }
  pub fn test<F>(&mut self, check: F) -> Result<(), Error<'a>>
  where
    F: Fn(&mut Self) -> Result<bool, Error<'a>>,
  {
    if check(self)? {
      self.cursor.next_char();
      Ok(())
    } else {
      Err(Error::new(
        ErrorKind::UnexpectedToken,
        self.cursor.span(),
        self.cursor.source,
      ))
    }
  }
  pub(crate) fn read_number(&mut self) -> Result<Token<'a>, Error<'a>> {
    self.test(|lexer| lexer.is_number_start())?;
    let mut is_float = false;
    let mut has_error = false;
    while !self.cursor.eof() && self.is_number()? {
      if self.cursor.peek() == '.' {
        if is_float {
          has_error = true;
        }
        is_float = true
      }
      self.cursor.next_char();
    }
    if has_error {
      return Err(Error::new(
        ErrorKind::TooManyFloatingPoints,
        self.cursor.span(),
        self.cursor.source,
      ));
    }
    let span = self.cursor.span();
    self.cursor.clear_span();
    let slice = span.expand();

    if is_float {
      Ok(Token::Float(slice))
    } else {
      Ok(Token::Integer(slice))
    }
  }
  pub fn consume(&mut self, char: char) -> Result<(), Error<'a>> {
    if self.cursor.next_char() != char {
      Err(Error::new(
        ErrorKind::UnexpectedToken,
        self.cursor.span(),
        self.cursor.source,
      ))
    } else {
      Ok(())
    }
  }
  pub fn read_keyword(&mut self) -> Result<Token<'a>, Error<'a>> {
    let id = self.read_id()?;
    match id {
      "function" => Ok(Token::Function),
      "module" => Ok(Token::Module),
      "public" => Ok(Token::Public),
      "let" => Ok(Token::Let),
      "mutable" => Ok(Token::Mutable),
      "import" => Ok(Token::Import),
      "while" => Ok(Token::While),
      "true" => Ok(Token::True),
      "false" => Ok(Token::False),
      _ => Ok(Token::Identifier(id)),
    }
  }
  pub fn read_single_char(&mut self) -> Result<Token<'a>, Error<'a>> {
    let token = match self.cursor.next_char() {
      '+' => Ok(Token::Plus),
      '-' => {
        if self.cursor.lookup(1)? == '>' {
          Ok(Token::Arrow)
        } else {
          Ok(Token::Minus)
        }
      }
      '*' => Ok(Token::Multiply),
      '/' => Ok(Token::Divide),
      ':' => Ok(Token::Colon),
      ';' => Ok(Token::Semicolon),
      '(' => Ok(Token::LeftRoundBrackets),
      ')' => Ok(Token::RightRoundBrackets),
      '{' => Ok(Token::LeftCurlyBrackets),
      '}' => Ok(Token::RightCurlyBrackets),
      '[' => Ok(Token::LeftSquareBrackets),
      ']' => Ok(Token::RightSquareBrackets),
      '"' => {
        while self.cursor.next_char() != '"' {}
        Ok(Token::String(self.cursor.span().expand()))
      }
      '>' => {
        if self.cursor.lookup(1)? == '=' {
          self.cursor.next_char();
          Ok(Token::GreeterEqual)
        } else {
          Ok(Token::Greeter)
        }
      }
      '<' => {
        if self.cursor.lookup(1)? == '=' {
          self.cursor.next_char();
          Ok(Token::LessEqual)
        } else {
          Ok(Token::Less)
        }
      }
      '=' => {
        if self.cursor.lookup(1)? == '=' {
          self.cursor.next_char();
          Ok(Token::Equal)
        } else {
          Ok(Token::Assignment)
        }
      }
      _ => Err(Error::new(
        ErrorKind::UnexpectedToken,
        self.cursor.span(),
        self.cursor.source,
      )),
    };
    self.cursor.clear_span();
    token
  }
  pub fn next_token(&mut self) -> Result<Token<'a>, Error<'a>> {
    if self.is_id_start()? {
      return self.read_keyword();
    }
    if self.is_number_start()? {
      return self.read_number();
    }
    self.read_single_char()
  }
  pub fn next_token_option(&mut self) -> Option<Result<Token<'a>, Error<'a>>> {

    self.skip().ok()?;
    if self.cursor.eof() {
      None
    } else {
      Some(self.next_token())
    }
  }
}
