use crate::lexer::TokenType;

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    for i in tokens {
        println!("\n {:?}", i);
    }
}