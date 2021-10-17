use ast::{Expression, Node, Operator, TopLevel, UnaryOperator};
use lexer::TokenKind;
use source::Source;

pub mod source;
pub struct Parser<'a> {
  pub(crate) source: Source<'a>,
}

impl<'a> Parser<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      source: Source::new(source),
    }
  }
  pub fn parse(&mut self) -> Result<TopLevel<'a>, String> {
    Ok(TopLevel::Expression(self.parse_expression()?))
  }
  pub fn parse_expression(&mut self) -> Result<Node<'a>, String> {
    self.parse_expression_with_binding_power(0)
  }
  // Based on https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
  pub fn parse_expression_with_binding_power(
    &mut self,
    minimal_binding_power: u8,
  ) -> Result<Node<'a>, String> {
    let next_token = self.source.next_token()?;
    let mut lhs = match next_token.kind {
      TokenKind::Number => Node::Integer(next_token.value),
      TokenKind::Float => Node::Float(next_token.value),
      TokenKind::Identifier => Node::Identifier(next_token.value),
      TokenKind::Plus | TokenKind::Minus => {
        let expression = Self::prefix_binding_power(&next_token.kind).unwrap();
        let operator = match next_token.kind {
          TokenKind::Minus => UnaryOperator::Minus,
          _ => unreachable!(),
        };
        let expr = self.parse_expression_with_binding_power(expression.1)?;
        Node::Expression(Expression::Unary {
          operator,
          argument: Box::new(expr),
        })
      }
      TokenKind::LeftParentheses => {
        let expression = self.parse_expression_with_binding_power(0)?;
        self.source.consume(TokenKind::RightParentheses)?;
        expression
      }
      _ => return Err("Bad token in left side of the expression".to_string()),
    };

    loop {
      let peek = self.source.peek();
      let token = match peek {
        Ok(token) => token,
        Err(_) => break,
      };
      if let Some((left_binding_power, right_binding_power)) =
        Self::infix_binding_power(&token.kind)
      {
        if left_binding_power < minimal_binding_power {
          break;
        }
        let operator = match token.kind {
          TokenKind::Plus => Operator::Plus,
          TokenKind::Minus => Operator::Minus,
          TokenKind::Multiply => Operator::Multiply,
          TokenKind::Divide => Operator::Divide,
          _ => break,
        };
        self.source.next_token()?;
        let rhs = self.parse_expression_with_binding_power(right_binding_power)?;
        lhs = Node::Expression(Expression::Binary {
          lhs: Box::new(lhs),
          rhs: Box::new(rhs),
          operator,
        });
        continue;
      }

      break;
    }
    Ok(lhs)
  }
  pub fn infix_binding_power(operator: &TokenKind) -> Option<(u8, u8)> {
    match operator {
      TokenKind::Plus | TokenKind::Minus => Some((1, 2)),
      TokenKind::Multiply | TokenKind::Divide => Some((2, 3)),
      _ => None,
    }
  }
  pub fn prefix_binding_power(operator: &TokenKind) -> Option<((), u8)> {
    match operator {
      TokenKind::Plus | TokenKind::Minus => Some(((), 5)),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use ast::{Expression, Node, Operator, TopLevel, UnaryOperator};

  use crate::Parser;

  fn check(input: &str, output: TopLevel) {
    let mut parser = Parser::new(input);
    assert_eq!(parser.parse().unwrap(), output);
  }
  #[test]
  fn can_parse_group() {
    check("(1)", TopLevel::Expression(Node::Integer("1")))
  }
  #[test]
  fn can_parse_id() {
    check("pi", TopLevel::Expression(Node::Identifier("pi")))
  }
  #[test]
  fn can_parse_float() {
    check("1.2", TopLevel::Expression(Node::Float("1.2")))
  }
  #[test]
  fn can_parse_int() {
    check("1", TopLevel::Expression(Node::Integer("1")))
  }

  #[test]
  fn can_parse_prefix_expression() {
    check(
      "-1",
      TopLevel::Expression(Node::Expression(Expression::Unary {
        operator: UnaryOperator::Minus,
        argument: Box::new(Node::Integer("1")),
      })),
    );
  }
  #[test]
  fn can_parse_infix_expression() {
    check(
      "1 - 2",
      TopLevel::Expression(Node::Expression(Expression::Binary {
        operator: Operator::Minus,
        lhs: Box::new(Node::Integer("1")),
        rhs: Box::new(Node::Integer("2")),
      })),
    );
  }
}
