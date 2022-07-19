mod ast;
mod lexer;

fn main() {
    let test_input = "int x = 3;";
    let mut lexer = lexer::Lexer::new();
    lexer.analyze(test_input);
    println!("{}", "0.4".parse::<f32>().unwrap());
}