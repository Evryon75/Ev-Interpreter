use crate::lexer::{NumericLiteralType, TokenType};

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    for i in tokens {
        println!("\n {:?}", i);
    }
}

struct AbstractSyntaxTree {
    program: Vec<Node>
}

enum Node {
    NumericLiteral{number_type: NumericLiteralType, value: i64},
    Subtraction{left: Node, right: Node},
    Addition{left: Node, right: Node},
    Multiplication{left: Node, right: Node},
    Division{left: Node, right: Node},
    Expression,
    Statement,
}