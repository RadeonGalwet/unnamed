use crate::common::error::Error;

use super::{token::Token, Lexer};

impl<'a> Iterator for Lexer<'a> {
  type Item = Result<Token<'a>, Error<'a>>;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_token_option()
  }
}
