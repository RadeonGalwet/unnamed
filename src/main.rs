use unnamed::{
  common::{source::Source, utils::get_utf8_slice},
  lexer::{cursor::Cursor, Lexer},
  parser::Parser,
};

fn main() {
  let source = Source {
    path: "main.ul",
    code: r#"
    let a = 2 * 2;
    print(a);
    "#,
  };
  let cursor = Cursor::new(source);
  let lexer = Lexer::new(cursor);
  let mut parser = Parser::new(lexer.peekable(), source, &cursor);
  let ast = parser.parse();
  match ast {
    Ok(node) => println!("{:#?}", node),
    Err(err) => println!(
      "{}\n{:?}:{}:{}",
      get_utf8_slice(err.source.code, err.span.start, err.span.end).unwrap(),
      err.kind,
      err.span.start,
      err.span.end
    ),
  }
}
