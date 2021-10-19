use enum_as_inner::EnumAsInner;

#[derive(Debug, PartialEq)]
pub struct Type<'a> {
  pub name: &'a str,
}
#[derive(Debug, PartialEq)]
pub struct Argument<'a> {
  pub name: &'a str,
  pub argument_type: Type<'a>,
}
#[derive(Debug, PartialEq)]
pub struct Function<'a> {
  pub name: &'a str,
  pub arguments: Vec<Argument<'a>>,
  pub body: Box<Node<'a>>,
  pub return_type: Type<'a>,
}
#[derive(Debug, PartialEq)]
pub enum Operator {
  Plus,
  Minus,
  Multiply,
  Divide,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
  Minus,
}
#[derive(Debug, PartialEq)]
pub enum TopLevel<'a> {
  Items(Vec<TopLevelItem<'a>>),
}
#[derive(Debug, PartialEq)]
pub enum TopLevelItem<'a> {
  Function {
    name: &'a str,
    arguments: Vec<Argument<'a>>,
    body: Box<Node<'a>>,
    return_type: Type<'a>,
  },
}
#[derive(Debug, PartialEq, EnumAsInner)]
pub enum Node<'a> {
  Identifier(&'a str), // Used only in expressions
  Integer(&'a str),
  Float(&'a str),
  Expression(Expression<'a>),
  Block(Vec<Node<'a>>),
  Statement(Statement<'a>),
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
  Call {
    name: Box<Node<'a>>,
    arguments: Vec<Node<'a>>
  }
}

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
  Return(Option<Box<Node<'a>>>),
}
