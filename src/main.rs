use unnamed::{lexer::Lexer, common::source::Source};

fn main() {
  let lexer = Lexer::new(Source {
    path: "src/main.ul",
    code: "abcd + abcd * abcd 2.2.2"
  });
  for token in lexer {
    if let Err(err) = token {
      println!("{:?} \"{} {}:{}\"", err.kind, err.source.path, err.span.start, err.span.end)
    } else {
      println!("{:?}", token);

    }
  }
}