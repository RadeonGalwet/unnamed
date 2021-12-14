use crate::{error::{LexingError}, token::Token, Lexer};

impl<'a> Iterator for Lexer<'a> {
  type Item = Result<Token<'a>, LexingError>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cursor.eof() {
      None
    } else {
      self.next_token_option()
    }
  }
}
