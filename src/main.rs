extern crate core;
use crate::ast::parse_tokens;
use crate::lexer::tokenize;
use crate::walker::walk;
use colour::*;
use std::fs::File;
use std::io::stdin;
use std::io::Read;

mod ast;
mod lexer;
mod walker;

fn main() {
    let mut file_input = File::open("main.ev").expect("main.ev File not found!");
    let mut reading_input: String = "".to_string();
    file_input
        .read_to_string(&mut reading_input)
        .expect("Failed to read from main.ev");

    let tokens = tokenize(reading_input).unwrap();
    let ast = parse_tokens(tokens.clone());
    walk(ast.clone()); //This was a one-liner but i needed these fields for debug information, and its cleaner too

    green_ln!("Process: finished successfully ✔");
    grey_ln!("Press [ENTER] to close, or input \"DEBUG\" to access debug information");
    let mut close_terminal = String::new();
    stdin().read_line(&mut close_terminal).expect("Something went wrong when taking input");
    if close_terminal == "DEBUG\n".to_string() {
        while !close_terminal.eq("EXIT\n") {
            grey_ln!("Input \"TOKENS\" to see the stream of tokens\nInput \"AST\" to show the abstract syntax tree\nInput \"EXIT\" to exit the program");
            close_terminal = "".to_string();
            stdin().read_line(&mut close_terminal).expect("Something went wrong when taking input");
            if close_terminal == "TOKENS\n" {
                let mut index = 0;
                for i in &tokens {
                    magenta_ln!("Index: {} > Value: {:#?}", index,  i);
                    index += 1;
                }
            } else if close_terminal == "AST\n" {
                for i in &ast.program {
                    yellow_ln!("Node: {:#?}", i);
                }
            }
        }
    }
}
