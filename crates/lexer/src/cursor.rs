use crate::{span::Span, error::{LexingError, LexingErrorKind}};

#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
  position: usize,
  start: usize,
  pub(crate) input: &'a str,
}

impl<'a> Cursor<'a> {
  pub fn new(input: &'a str) -> Self {
    Self {
      position: 0,
      start: 0,
      input,
    }
  }
  pub fn next(&mut self) -> Result<char, LexingError> {
    let char = self.peek()?;
    self.position += 1;
    Ok(char)
  }
  pub fn bump(&mut self) -> Result<char, LexingError> {
    self.position += 1;
    self.peek()
  }
  pub fn peek(&self) -> Result<char, LexingError> {
    
    if self.eof() {
      return Err(LexingError::new(LexingErrorKind::UnexpectedEndOfInput, self.span()))
    }
    if self.position == 0 {
      self.input[0..1]
        .chars()
        .nth(0)
        .ok_or_else(|| LexingError::new(LexingErrorKind::UnexpectedEndOfInput, self.span()))
    } else {
      self.input[self.position..self.position + 1]
        .chars()
        .nth(0)
        .ok_or_else(|| LexingError::new(LexingErrorKind::UnexpectedEndOfInput, self.span()))
    }
  }
  pub fn span(&self) -> Span {
    Span {
      start: self.start,
      end: self.position,
    }
  }
  pub fn clear_span(&mut self) {
    self.start = self.position;
  }
  pub fn eof(&self) -> bool {
    self.position > (self.input.len() - 1)
  } 
}