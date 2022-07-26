use crate::lexer::{DeclarationKeywords, NumericLiteralType, TokenType};

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    for i in &tokens {
        println!("RAW: {:?}", i);
    }

    let mut cursor = 0;
    let mut expect = |token_type: TokenType| {
        if std::mem::discriminant(&tokens[cursor]) == std::mem::discriminant(&token_type) {
            cursor += 1;
        } else {
            println!("Expected: {:?} Found: {:?}", token_type,  &tokens[cursor]);
            std::process::exit(0);
        }
    };

    //Basic number variable declaration test
    expect(TokenType::DeclarationKeyword { keyword: DeclarationKeywords::Let });
    expect(TokenType::Identifier { identifier: "".to_string() });
    expect(TokenType::Equal);
    expect(TokenType::NumericLiteral { numeric_type: NumericLiteralType::Int, value: 0.0 });

    let ast = AbstractSyntaxTree { program: vec![] };
    for i in ast.program {
        println!("{:?}", i);
    }
}
fn parse_expression() {

}

struct AbstractSyntaxTree {
    pub program: Vec<Node>,
}
impl AbstractSyntaxTree {}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Min,
    Mul,
    Div
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
    Expression
}
