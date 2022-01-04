use super::{
  ast::{expression::InfixOperator, CalculateSpan, Node, Spanned},
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
  fn infix_binding_power(kind: TokenKind) -> Option<(u8, u8)> {
    match kind {
      TokenKind::Plus | TokenKind::Minus => Some((1, 2)),
      TokenKind::Multiply | TokenKind::Divide => Some((3, 4)),
      _ => None,
    }
  }
  fn postfix_binding_power(kind: TokenKind) -> Option<(u8, ())> {
    match kind {
      TokenKind::LeftRoundBracket => Some((5, ())),
      _ => None,
    }
  }
  pub fn expression(&mut self, minimal_binding_power: u8) -> Result<'a, Node<'a>> {
    let next_token = self.next_token()?;
    let mut lhs = match next_token.kind {
      TokenKind::Integer => Node::Integer(Spanned::new(next_token.value()?, next_token.span)),
      TokenKind::Float => Node::Float(Spanned::new(next_token.value()?, next_token.span)),
      TokenKind::Identifier => Node::Identifier(Spanned::new(next_token.value()?, next_token.span)),
      TokenKind::LeftRoundBracket => {
        let expression = self.expression(0)?;
        self.consume(TokenKind::RightRoundBracket)?;
        expression
      }
      _ => {
        return Err(Error::new(
          ErrorKind::UnexpectedToken,
          self.source,
          next_token.span,
        ))
      }
    };
    loop {
      let token = self.peek();
      let token = match token {
        Ok(token) => token,
        Err(_) => break,
      };
      if let Some((left_binding_power, right_binding_power)) = Self::infix_binding_power(token.kind)
      {
        if left_binding_power < minimal_binding_power {
          break;
        }
        let operator = match token.kind {
          TokenKind::Plus => InfixOperator::Plus,
          TokenKind::Minus => InfixOperator::Minus,
          TokenKind::Multiply => InfixOperator::Multiply,
          TokenKind::Divide => InfixOperator::Divide,
          _ => break,
        };
        self.next_token()?;
        let rhs = self.expression(right_binding_power)?;
        lhs = Node::Expression(Expression::Binary {
          operator,
          lhs: box lhs,
          rhs: box rhs,
        });
        continue;
      } else if let Some((left_binding_power, ())) = Self::postfix_binding_power(token.kind) {
        if left_binding_power < minimal_binding_power {
          break;
        }
        match token.kind {
          TokenKind::LeftRoundBracket => {
            let name = match lhs {
              Node::Identifier(spanned_value) => spanned_value,
              _ => {
                return Err(Error::new(
                  ErrorKind::UnexpectedToken,
                  self.source,
                  lhs.calculate_span(),
                ))
              }
            };
            self.next_token()?;
            let arguments = self.arguments(|parser| parser.expression(0), None);
            self.consume(TokenKind::RightRoundBracket)?;
            lhs = Node::Expression(Expression::Call {
              name,
              arguments: arguments?,
            });
          }
          _ => break,
        }
        continue;
      }
      break;
    }
    Ok(lhs)
  }
}
