use unnamed::{
  common::{source::Source, utils::get_utf8_slice},
  lexer::{cursor::Cursor, Lexer},
  parser::Parser,
};

fn main() {
  let source = Source {
    path: "main.ul",
    code: "1 * 2 + 3",
  };
  let mut cursor = Cursor::new(source);
  let lexer = Lexer::new(cursor);
  let mut parser = Parser::new(lexer.peekable(), source, &mut cursor);
  let span = parser.expression(0).unwrap().calculate_span();
  println!(
    "{}",
    get_utf8_slice(source.code, span.start, span.end).unwrap()
  );
}
