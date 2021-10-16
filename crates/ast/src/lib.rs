#[derive(Debug, PartialEq)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
  Plus,
  Minus,
}
#[derive(Debug, PartialEq)]
pub enum TopLevel<'a> {
  Expression(Node<'a>),
}
#[derive(Debug, PartialEq)]
pub enum Node<'a> {
  Identifier(&'a str),
  Integer(&'a str),
  Float(&'a str),
  Expression(Expression<'a>),
}
#[derive(Debug, PartialEq)]

pub enum Expression<'a> {
  Binary {
    operator: Operator,
    lhs: Box<Node<'a>>,
    rhs: Box<Node<'a>>,
  },
  Unary {
    operator: UnaryOperator,
    argument: Box<Node<'a>>,
  },
}
