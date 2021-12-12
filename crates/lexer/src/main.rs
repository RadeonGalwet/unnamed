use std::ops::Range;

use lexer::Lexer;

fn main() {
  let input = r#"
  public function main() -> integer {
    let mutable i = 0;
    while(i < 100) {
      print("{i}");
    }
    print("end");
  }
  "#;
  let mut lexer = Lexer::new(input);
  while !lexer.cursor.eof() {
    let token = lexer.next_token();
    match token {
        Ok(token) => println!("{:?}", token),
        Err(err) => eprintln!("{} at {}:{}\n\ninput = {:?}", err, err.span.start, err.span.end, &input[err.span.start - 1..err.span.end]),
    }
  }
}