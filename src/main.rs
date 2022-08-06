extern crate core;
use crate::ast::parse_tokens;
use crate::lexer::tokenize;
use crate::walker::walk;
use std::fs::File;
use std::io::Read;

mod ast;
mod lexer;
mod walker;

fn main() {
    let mut file_input = File::open("main.ev").expect("Something went wrong");
    let mut reading_input: String = "".to_string();

    file_input
        .read_to_string(&mut reading_input)
        .expect("Main file not found");
    walk(parse_tokens(tokenize(reading_input).unwrap()));
}
