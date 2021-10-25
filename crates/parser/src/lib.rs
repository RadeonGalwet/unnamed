use ast::{
  Argument, Expression, Function, Node, Operator, Statement, TopLevel, Type, UnaryOperator,
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
    let mut functions = vec![];
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
                name: id,
                argument_type: Type { name: type_name },
              })
            },
            Some(TokenKind::Identifier),
          )?;
          self.source.consume(TokenKind::RightParentheses)?;
          let return_type = if self.source.test_and_next(TokenKind::Arrow)? {
            Type {
              name: self.source.consume(TokenKind::Identifier)?.value,
            }
          } else {
            Type { name: "void" }
          };
          let body = if self.source.test_and_next(TokenKind::Assignment)? {
            let expression = self.parse_expression()?;
            self.source.consume(TokenKind::SemiColon)?;
            expression
          } else {
            self.parse_block()?
          };
          functions.push(Function {
            name: identifier,
            arguments,
            return_type,
            body: Box::new(body),
          });
        }
        _ => return Err(format!("Expected function, found {}", token.value)),
      };
    }

    Ok(TopLevel { functions })
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
          let expression = self.parse_expression()?;
          self.source.consume(TokenKind::SemiColon)?;
          Ok(Node::Statement(Statement::Return(Some(Box::new(
            expression,
          )))))
        }
      }
      TokenKind::LeftCurly => self.parse_block(),
      TokenKind::If => {
        self.source.next_token()?;
        let expression = self.parse_expression()?;
        let then_branch = if self.source.test(TokenKind::LeftCurly) {
          let block = self.parse_block()?;
          block
        } else {
          let expression = self.parse_expression()?;
          self.source.consume(TokenKind::SemiColon)?;
          expression
        };
        let else_branch = if self.source.test_and_next(TokenKind::Else)? {
          Some(Box::new(self.statement()?))
        } else {
          None
        };
        Ok(Node::Statement(Statement::Conditional {
          then_branch: Box::new(then_branch),
          expression: Box::new(expression),
          else_branch,
        }))
      }
      TokenKind::Let => {
        self.source.next_token()?;
        let mutable = self.source.test_and_next(TokenKind::Mut)?;
        let id = self.source.consume(TokenKind::Identifier)?.value;
        let init_type = if self.source.test_and_next(TokenKind::Colon)? {
          Some(self.source.consume(TokenKind::Identifier)?.value)
        } else {
          None
        };
        let init = if self.source.test_and_next(TokenKind::Assignment)? {
          let expression = self.parse_expression()?;
          self.source.consume(TokenKind::SemiColon)?;
          Some(Box::new(expression))
        } else {
          None
        };
        Ok(Node::Statement(Statement::LetBinding {
          id,
          mutable,
          init,
          init_type,
        }))
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
    self.source.consume(TokenKind::LeftCurly)?;
    while !self.source.test(TokenKind::RightCurly) && self.source.peek().is_ok() {
      statements.push(self.statement()?)
    }
    self.source.consume(TokenKind::RightCurly)?;
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
      TokenKind::True => Node::Boolean(true),
      TokenKind::False => Node::Boolean(false),

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
          TokenKind::Equal => Operator::Equal,
          TokenKind::NotEqual => Operator::NotEqual,
          TokenKind::Less => Operator::Less,
          TokenKind::LessEqual => Operator::LessEqual,
          TokenKind::Greater => Operator::Greater,
          TokenKind::GreaterEqual => Operator::GreaterEqual,
          TokenKind::And => Operator::And,
          TokenKind::Or => Operator::Or,
          TokenKind::Assignment => Operator::Assignment,
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
      if let Some((left_binding_power, ())) = Self::postfix_binding_power(&token.kind) {
        if left_binding_power < minimal_binding_power {
          break;
        }
        self.source.next_token()?;
        let arguments = self.arguments(|parser| parser.parse_expression(), None)?;
        self.source.consume(TokenKind::RightParentheses)?;
        if lhs.as_identifier().is_some() {
          lhs = Node::Expression(Expression::Call {
            name: next_token.value,
            arguments,
          })
        }
        continue;
      }
      break;
    }
    Ok(lhs)
  }
  pub fn infix_binding_power(operator: &TokenKind) -> Option<(u8, u8)> {
    match operator {
      TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual => {
        Some((1, 2))
      }
      TokenKind::And => Some((2, 3)),
      TokenKind::Or => Some((3, 4)),
      TokenKind::Equal | TokenKind::NotEqual => Some((4, 5)),
      TokenKind::Plus | TokenKind::Minus => Some((5, 6)),
      TokenKind::Multiply | TokenKind::Divide => Some((7, 8)),
      TokenKind::Assignment => Some((8, 9)),
      _ => None,
    }
  }
  pub fn prefix_binding_power(operator: &TokenKind) -> Option<((), u8)> {
    match operator {
      TokenKind::Minus => Some(((), 7)),
      _ => None,
    }
  }
  pub fn postfix_binding_power(operator: &TokenKind) -> Option<(u8, ())> {
    match operator {
      TokenKind::LeftParentheses => Some((10, ())),
      _ => None,
    }
  }
}

#[cfg(test)]
mod tests {
  use ast::{
    Argument, Expression, Function, Node, Operator, Statement, TopLevel, Type, UnaryOperator,
  };

  use crate::Parser;

  fn check(input: &str, output: TopLevel) {
    let mut parser = Parser::new(input);
    assert_eq!(parser.parse().unwrap(), output);
  }
  #[test]
  fn can_parse_function() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {}",
      TopLevel {
        functions: vec![function],
      },
    )
  }
  #[test]
  fn can_parse_inline_function() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Expression(Expression::Binary {
        lhs: Box::new(Node::Integer("2")),
        rhs: Box::new(Node::Integer("2")),
        operator: Operator::Plus,
      })),
      return_type: Type { name: "void" },
    };
    check(
      "function main() = 2 + 2;",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_function_with_arguments() {
    let function = Function {
      name: "main",
      arguments: vec![Argument {
        name: "a",
        argument_type: Type { name: "i32" },
      }],
      body: Box::new(Node::Block(vec![])),
      return_type: Type { name: "void" },
    };
    check(
      "function main(a: i32) {}",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_function_with_multiple_arguments() {
    let function = Function {
      name: "main",
      arguments: vec![
        Argument {
          name: "a",
          argument_type: Type { name: "i32" },
        },
        Argument {
          name: "b",
          argument_type: Type { name: "i32" },
        },
      ],
      body: Box::new(Node::Block(vec![])),
      return_type: Type { name: "void" },
    };
    check(
      "function main(a: i32, b: i32) {}",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_binary_expression() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Binary {
        lhs: Box::new(Node::Integer("1")),
        rhs: Box::new(Node::Integer("2")),
        operator: Operator::Plus,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      1 + 2;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_binary_expression_with_logical() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Binary {
        lhs: Box::new(Node::Integer("1")),
        rhs: Box::new(Node::Integer("2")),
        operator: Operator::Equal,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      1 == 2;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_float() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Binary {
        lhs: Box::new(Node::Float("1.1")),
        rhs: Box::new(Node::Float("2.2")),
        operator: Operator::Plus,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      1.1 + 2.2;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_boolean() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Binary {
        lhs: Box::new(Node::Boolean(true)),
        rhs: Box::new(Node::Boolean(false)),
        operator: Operator::And,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      true && false;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_group() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Binary {
        lhs: Box::new(Node::Float("1.1")),
        rhs: Box::new(Node::Float("2.2")),
        operator: Operator::Plus,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      (1.1 + 2.2);
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_unary_expression() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Unary {
        argument: Box::new(Node::Integer("1")),
        operator: UnaryOperator::Minus,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      -1;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_unary_expression_with_float() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Expression(Expression::Unary {
        argument: Box::new(Node::Float("1.1")),
        operator: UnaryOperator::Minus,
      })])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      -1.1;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
  #[test]
  fn can_parse_return() {
    let function = Function {
      name: "main",
      arguments: vec![],
      body: Box::new(Node::Block(vec![Node::Statement(Statement::Return(Some(
        Box::new(Node::Float("1.1")),
      )))])),
      return_type: Type { name: "void" },
    };
    check(
      "function main() {
      return 1.1;
    }",
      TopLevel {
        functions: vec![function],
      },
    );
  }
}
