use ast::{
  Argument, Expression, Node, Operator, Statement, TopLevel, TopLevelItem, Type, UnaryOperator,
};
use lexer::TokenKind;
use source::Source;
use state::State;
pub mod source;
pub mod state;

pub struct Parser<'a> {
  pub(crate) source: Source<'a>,
  pub(crate) state: State,
}

impl<'a> Parser<'a> {
  pub fn new(source: &'a str) -> Self {
    Self {
      source: Source::new(source),
      state: State::None,
    }
  }
  pub fn update_state(&mut self, state: State) {
    self.state = state;
  }
  pub fn parse(&mut self) -> Result<TopLevel<'a>, String> {
    self.parse_top_level()
  }
  pub fn arguments<F, T>(&mut self, function: F, test: Option<TokenKind>) -> Result<Vec<T>, String>
  where
    F: Fn(&mut Self) -> Result<T, String>,
  {
    let mut args = vec![];
    loop {
      if let Some(test) = test {
        if self.source.test(test) {
          args.push(function(self)?);
        } else {
          break;
        }
      } else {
        args.push(function(self)?);
      }
      if !self.source.test_and_next(TokenKind::Comma)? {
        break;
      }
    }
    Ok(args)
  }
  pub fn parse_top_level(&mut self) -> Result<TopLevel<'a>, String> {
    let mut items = vec![];
    while self.source.peek().is_ok() {
      let token = self.source.next_token()?;
      match token.kind {
        TokenKind::Function => {
          self.update_state(State::Function);
          let identifier = self.source.consume(TokenKind::Identifier)?.value;
          self.source.consume(TokenKind::LeftParentheses)?;
          let arguments = self.arguments(
            |parser| {
              let id = parser.source.consume(TokenKind::Identifier)?.value;
              parser.source.consume(TokenKind::Colon)?;
              let type_name = parser.source.consume(TokenKind::Identifier)?.value;
              Ok(Argument {
                name: Box::new(Node::Identifier(id)),
                type_name: Type {
                  name: Box::new(Node::Identifier(type_name)),
                  type_arguments: None,
                },
              })
            },
            Some(TokenKind::Identifier),
          )?;
          self.source.consume(TokenKind::RightParentheses)?;
          let return_type = if self.source.test_and_next(TokenKind::Arrow)? {
            Type {
              name: Box::new(Node::Identifier(
                self.source.consume(TokenKind::Identifier)?.value,
              )),
              type_arguments: None,
            }
          } else {
            Type {
              name: Box::new(Node::Identifier("void")),
              type_arguments: None,
            }
          };
          let body = if self.source.test_and_next(TokenKind::Assignment)? {
            let expression = self.parse_expression()?;
            self.source.consume(TokenKind::SemiColon)?;
            expression
          } else {
            self.parse_block()?
          };
          items.push(TopLevelItem::Function {
            name: Box::new(Node::Identifier(identifier)),
            arguments,
            return_type,
            body: Box::new(body),
          });
        }
        _ => return Err(format!("Expected function, found {}", token.value)),
      };
    }

    Ok(TopLevel::Items(items))
  }
  pub fn statement(&mut self) -> Result<Node<'a>, String> {
    let token = self.source.peek()?;
    match token.kind {
      TokenKind::Return => {
        self.source.next_token()?;
        if self.state != State::Function {
          return Err("Return outside of function".to_string());
        }
        if self.source.test_and_next(TokenKind::SemiColon)? {
          Ok(Node::Statement(Statement::Return(None)))
        } else {
          println!("{:?}", self.source.peek());
          let expression = self.parse_expression()?;
          self.source.consume(TokenKind::SemiColon)?;
          Ok(Node::Statement(Statement::Return(Some(Box::new(
            expression,
          )))))
        }
      }
      _ => {
        let expression = self.parse_expression()?;
        self.source.consume(TokenKind::SemiColon)?;
        Ok(expression)
      }
    }
  }
  pub fn parse_block(&mut self) -> Result<Node<'a>, String> {
    let mut statements = vec![];
    self.source.consume(TokenKind::LeftBracket)?;
    while !self.source.test(TokenKind::RightBracket) && self.source.peek().is_ok() {
      statements.push(self.statement()?)
    }
    self.source.consume(TokenKind::RightBracket)?;
    Ok(Node::Block(statements))
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
      TokenKind::Minus => {
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
