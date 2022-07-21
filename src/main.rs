use crate::lexer::tokenize;

mod ast;
mod lexer;

fn main() {
    let test_input: String = "3 5".parse().unwrap();
    let tokenized_input = tokenize(test_input);
    for i in tokenized_input.unwrap() {
        println!("\n {:?}", i);
    }
}