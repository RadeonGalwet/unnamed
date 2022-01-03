use std::time::Instant;

use unnamed::{lexer::{Lexer}, common::source::Source};

fn main() {
  let mut lexer = Lexer::new(Source {
    code: r#"
    let a = 2; // a is height
    let b = 3; // b is width
    /*
      Formula:
      a * b = z
    */
    let z = a * b;
    print(z); // print the input
    "#,   
    path: "main.ul"
  });
  let mut buffer = vec![];
  while let Ok(token) = lexer.next_token() {
    buffer.push(token.value().unwrap())
  }
  println!("{}", buffer.join(" "))
  
}
