use super::{Lexer, Token};
use crate::common::result::Result;
impl<'a> Iterator for Lexer<'a> {
  type Item = Result<'a, Token<'a>>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cursor.eof() {
      None
    } else {
      Some(self.next_token())
    }
  }
}
