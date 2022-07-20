mod ast;
mod lexer;

fn main() {
    let test_input: String = "float dubble = (num) {num * 2} dubble(3);".parse().unwrap();
    let lexer = lexer::Lexer::new(test_input);
    for i in lexer.tokens {
        println!("{:?}", i);
    }
}