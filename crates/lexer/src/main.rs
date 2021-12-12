use lexer::Lexer;

fn main() {
  let mut lexer = Lexer::new("a * 3.4.4 - b");
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
  println!("{:?}", lexer.next_token());
}