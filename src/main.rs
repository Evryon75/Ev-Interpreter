mod ast;
mod lexer;

fn main() {
    let test_input = "float dubble = (num) {num * 2} dubble(3);";
    let mut lexer = lexer::Lexer::new();
    lexer.analyze(test_input);
    for i in lexer.tokens {
        println!("{:?}", i);
    }
}