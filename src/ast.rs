use crate::lexer::{NumericLiteralType, TokenType};

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    for i in tokens {
        println!("\n {:?}", i);
    }

    let ast = AbstractSyntaxTree { program: vec![] };
    for i in ast.program {
        println!("{:?}", i);
    }
}

struct AbstractSyntaxTree {
    pub program: Vec<Node>,
}
impl AbstractSyntaxTree {}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
}
#[derive(Debug, PartialEq)]
enum Node {
    NumericLiteral {
        numeric_type: NumericLiteralType,
        value: f64,
    },
    UnaryExpression {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpression {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
