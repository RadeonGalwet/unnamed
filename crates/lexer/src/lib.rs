#![feature(decl_macro)]
use cursor::Cursor;
use error::{LexingError, LexingErrorKind};
use token::Token;

pub mod cursor;
pub mod error;
pub mod span;
pub mod token;
pub mod iterator;

pub struct Lexer<'a> {
  pub cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      cursor: Cursor::new(input),
    }
  }
  pub fn skip(&mut self) -> Result<(), LexingError> {
    while !self.cursor.eof() && (self.cursor.peek()? == '\n' || self.cursor.peek()? == ' ') {
      self.cursor.next_char()?;
    }
    self.cursor.clear_span();
    Ok(())
  }
  pub fn is_id_start(&mut self) -> Result<bool, LexingError> {
    let char = self.cursor.peek()?;
    Ok(('a'..='z').contains(&char) || ('A'..='Z').contains(&char) || char == '_' || char == '$')
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
      self.cursor.next_char()?;
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
      self.cursor.next_char()?;
      Ok(())
    } else {
      Err(LexingError::new(
        LexingErrorKind::UnexpectedToken,
        self.cursor.span(),
      ))
    }
  }
  pub(crate) fn read_number(&mut self) -> Result<Token<'a>, LexingError> {
    self.test(|lexer| lexer.is_number_start())?;
    let mut is_float = false;
    let mut has_error = false; 
    while !self.cursor.eof() && self.is_number()? {
      if self.cursor.peek()? == '.' {
        if is_float {
          has_error = true;
        }
        is_float = true
      }
      self.cursor.next_char()?;
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
  pub fn consume(&mut self, char: char) -> Result<(), LexingError> {
    if self.cursor.next_char()? != char {
      Err(LexingError::new(LexingErrorKind::UnexpectedToken, self.cursor.span()))
    } else {
      Ok(())
    }
  }
  pub fn read_keyword(&mut self) -> Result<Token<'a>, LexingError> {
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
      _ => Ok(Token::Identifier(id))
    }
  }
  pub fn read_single_char(&mut self) -> Result<Token<'a>, LexingError> {
    let token = match self.cursor.next_char()? {
      '+' => Ok(Token::Plus),
      '-' => {
        if self.cursor.lookup(1)? == '>' {
          Ok(Token::Arrow)
        } else {
          Ok(Token::Minus)
        }
      },
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
        while self.cursor.next_char()? != '"' {};
        Ok(Token::String(self.cursor.span().expand(&mut self.cursor)))
      },
      '>' => {
        if self.cursor.lookup(1)? == '=' {
          self.cursor.next_char()?;
          Ok(Token::GreeterEqual)
        } else {
          Ok(Token::Greeter)
        }
      },
      '<' => {
        if self.cursor.lookup(1)? == '=' {
          self.cursor.next_char()?;
          Ok(Token::LessEqual)
        } else {
          Ok(Token::Less)
        }
      }
      '=' => {
        if self.cursor.lookup(1)? == '=' {
          self.cursor.next_char()?;
          Ok(Token::Equal)
        } else {
          Ok(Token::Assignment)
        }
      }
      _ => Err(LexingError::new(
        LexingErrorKind::UnexpectedToken,
        self.cursor.span(),
      )),
    };
    self.cursor.clear_span();
    token
  }
  pub fn next_token(&mut self) -> Result<Token<'a>, LexingError> {    
    if self.is_id_start()? {
      return self.read_keyword()
    }
    if self.is_number_start()? {
      return self.read_number();
    }
    self.read_single_char()
  }
  pub fn next_token_option(&mut self) -> Option<Result<Token<'a>, LexingError>> {
    self.skip().ok()?;
    if self.cursor.eof() {
      None
    } else {
      Some(self.next_token())
    }
  }
}