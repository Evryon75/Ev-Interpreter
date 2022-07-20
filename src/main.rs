use crate::lexer::tokenize;

mod ast;
mod lexer;

fn main() {
    let test_input: String = "int x = 10;".parse().unwrap();
    tokenize(test_input);
}