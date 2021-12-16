#[derive(Debug, Clone, Copy)]
pub enum Token<'a> {
  /// Identifier "[a-zA-Z$_]+[a-zA-Z$_]*"
  Identifier(&'a str),
  /// Integer "[0-9]+"
  Integer(&'a str),
  /// Float "[0-9]+\.[0-9]+"
  Float(&'a str),
  /// String "\".+\""
  String(&'a str),

  /// "+" token
  Plus,
  /// "-" token
  Minus,
  /// "/" token
  Divide,
  /// "*" token
  Multiply,
  /// "<" token
  Less,
  /// "<=" token
  LessEqual,
  /// ">" token
  Greeter,
  /// ">=" token
  GreeterEqual,
  /// "=" token
  Assignment,
  /// "==" token
  Equal,
  /// ":" token
  Colon,
  /// ";" token
  Semicolon,
  /// "->" token
  Arrow,

  /// "(" bracket
  LeftRoundBrackets,
  /// ")" bracket
  RightRoundBrackets,
  /// "[" bracket
  LeftSquareBrackets,
  /// "]" bracket
  RightSquareBrackets,
  /// "{" bracket
  LeftCurlyBrackets,
  /// "}" bracket
  RightCurlyBrackets,

  /// "function" keyword
  Function,
  /// "module" keyword
  Module,
  /// "import" keyword
  Import,
  /// "let" keyword
  Let,
  /// "mutable" keyword
  Mutable,
  /// "public" keyword
  Public,
  /// "while" keyword
  While,
  /// "type" keyword
  Type,
  /// "true" literal
  True,
  /// "false" literal
  False,
}
