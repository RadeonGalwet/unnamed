use super::{
  ast::{Node, Spanned},
  Parser,
};
use crate::{common::result::Result, lexer::token::TokenKind, parser::ast::statement::Statement};
impl<'a, 'b> Parser<'a, 'b> {
  pub fn statement(&mut self) -> Result<'a, Node<'a>> {
    let token = self.peek()?;
    match token.kind {
      TokenKind::Let => {
        self.next_token()?;
        let name = self.consume(TokenKind::Identifier)?;
        self.consume(TokenKind::Assignment)?;
        let expression = self.expression(0)?;
        self.consume(TokenKind::Semicolon)?;
        Ok(Node::Statement(Statement::LetBinding {
          name: Spanned::new(name.value()?, name.span),
          value: box expression,
        }))
      }
      _ => {
        let expression = self.expression(0)?;
        self.consume(TokenKind::Semicolon)?;
        Ok(expression)
      }
    }
  }
  pub fn statements(&mut self) -> Result<'a, Vec<Node<'a>>> {
    let mut statements = vec![];
    while self.peek().is_ok() {
      statements.push(self.statement()?)
    }
    Ok(statements)
  }
}
