use crate::lexer::tokenize;

mod ast;
mod lexer;

fn main() {
    let test_input: String = "float dubble = (num) {num * 2} dubble(3);".parse().unwrap();
    tokenize(test_input);
}