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

    //Tribute to Somewhere City
    magenta_ln!(
        "Thank you for checking out this project! ✨\n\
    I learned a lot about language development and Rust.\n\
    This project was infused with the album ⪧\"Somewhere City\"\n\
    by ⪧\"Origami Angel\", give it a listen! 🎶\n\
    ⪼https://www.youtube.com/watch?v=lNosH1DEPkQ⪻"
    );

    white_ln!("Press [ENTER] to close, or input \"DEBUG\" to access debug information");
    grey_ln!("Both \"DEBUG\" and \"debug\" will work");
    let mut close_terminal = String::new();
    stdin()
        .read_line(&mut close_terminal)
        .expect("Something went wrong when taking input");
    if close_terminal.eq_ignore_ascii_case("DEBUG\n") {
        white_ln!("Input \"TOKENS\" to see the stream of tokens\nInput \"AST\" to show the abstract syntax tree\nInput \"EXIT\" to exit the program");
        while !close_terminal.eq_ignore_ascii_case("EXIT\n") {
            close_terminal = "".to_string();
            stdin()
                .read_line(&mut close_terminal)
                .expect("Something went wrong when taking input");
            if close_terminal.eq_ignore_ascii_case("TOKENS\n") {
                let mut index = 0;
                for i in &tokens {
                    magenta_ln!("Index: {} > Value: {:#?}", index, i);
                    index += 1;
                }
            } else if close_terminal.eq_ignore_ascii_case("AST\n") {
                for i in &ast.program {
                    yellow_ln!("Node: {:#?}", i);
                }
            }
            if !close_terminal.eq_ignore_ascii_case("EXIT\n") {
                grey_ln!("[TOKENS] [AST] [EXIT]");
            }
        }
    }
}
