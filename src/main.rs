extern crate core;

use crate::ast::parse_tokens;
use crate::lexer::tokenize;
use std::fs::File;
use std::io::Read;

mod ast;
mod lexer;
mod walker;

fn main() {
    let mut file_input = File::open("hello world.ev").expect("Something went wrong");
    let mut reading_input: String = "".to_string();

    file_input
        .read_to_string(&mut reading_input)
        .expect("Something went wrong when reading to string");
    parse_tokens(tokenize(reading_input).unwrap());
}
