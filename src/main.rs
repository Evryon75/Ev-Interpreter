mod ast;
mod lexer;

fn main() {
    let test_input = "int x = 3;";
    let mut lexer = lexer::Lexer::new();
    lexer.analyze(test_input);
    for i in lexer.tokens {
        println!("{:?}", i);
    }
}