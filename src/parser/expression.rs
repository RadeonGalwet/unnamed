use super::{
  ast::{expression::InfixOperator, Node},
  Parser,
};
use crate::{
  common::{
    error::{Error, ErrorKind},
    result::Result,
  },
  lexer::token::TokenKind,
  parser::ast::expression::Expression,
};

impl<'a, 'b> Parser<'a, 'b> {
  fn infix_binding_power(operator: InfixOperator) -> (u8, u8) {
    match operator {
      InfixOperator::Plus | InfixOperator::Minus => (1, 2),
      InfixOperator::Multiply | InfixOperator::Divide => (3, 4),
    }
  }
  pub fn expression(&mut self, minimal_binding_power: u8) -> Result<'a, Node<'a>> {
    let token = self.next()?;
    let mut lhs = match token.kind {
      TokenKind::Integer => Node::Integer {
        value: token.value()?,
        span: token.span,
      },
      TokenKind::Float => Node::Float {
        value: token.value()?,
        span: token.span,
      },
      _ => {
        return Err(Error::new(
          ErrorKind::UnexpectedToken,
          self.source,
          token.span,
        ))
      }
    };
    loop {
      let operator = match self.peek() {
        Ok(token) => match token.kind {
          TokenKind::Plus => InfixOperator::Plus,
          TokenKind::Minus => InfixOperator::Minus,
          TokenKind::Multiply => InfixOperator::Multiply,
          TokenKind::Divide => InfixOperator::Divide,
          _ => break,
        },
        Err(_) => break,
      };
      let (left_binding_power, right_binding_power) = Self::infix_binding_power(operator);
      if left_binding_power < minimal_binding_power {
        break;
      }
      self.next()?;
      let rhs = self.expression(right_binding_power)?;
      lhs = Node::Expression(Expression::Binary {
        operator: operator,
        lhs: box lhs,
        rhs: box rhs,
      })
    }
    Ok(lhs)
  }
}
