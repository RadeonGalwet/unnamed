use crate::common::{error::{Error, ErrorKind}, source::Source, span::Span};


#[derive(Clone, Copy, Debug)]
pub struct Cursor<'a> {
  position: usize,
  start: usize,
  pub(crate) source: Source<'a>,
}

impl<'a> Cursor<'a> {
  pub fn new(source: Source<'a>) -> Self {
    Self {
      position: 0,
      start: 0,
      source,
    }
  }
  pub fn next_char(&mut self) -> char {
    let char = self.peek();
    self.position += 1;
    char
  }
  pub fn bump(&mut self) -> char {
    self.position += 1;
    self.peek()
  }
  // Returns char because peek can't be called with eof
  pub fn peek(&self) -> char {    
    if self.position == 0 {
      self.source.code[0..1]
        .chars().next()
        .unwrap()
    } else {
      self.source.code[self.position..self.position + 1]
        .chars().next()
        .unwrap()
    }
  }
  pub fn span(&self) -> Span<'a> {
    Span {
      start: self.start,
      end: self.position,
      source: self.source
    }
  }
  pub fn clear_span(&mut self) {
    self.start = self.position;
  }
  pub fn lookup(&mut self, count: usize) -> Result<char, Error<'a>> {
    let lookup_position = self.position + count;
    if lookup_position > (self.source.len() - 1) {
      return Err(Error::new(ErrorKind::UnexpectedEndOfInput, self.span(), self.source))
    }
    Ok(self.source.code[self.position..self.position + count].chars().next().unwrap())
  }
  pub fn eof(&self) -> bool {
    self.position > (self.source.len() - 1)
  } 
}