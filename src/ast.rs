use crate::lexer::{DeclarationKeywords, NumericLiteralType, TokenType};

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    for i in &tokens {
        println!("RAW: {:?}", i);
    }

    let mut cursor = 0;
    let mut expect = |token_type: TokenType, cursor: &mut usize| {
        if std::mem::discriminant(&tokens[*cursor]) == std::mem::discriminant(&token_type) {
            *cursor += 1;
        } else {
            println!("Parsing Error: Unexpected Token [{:?}]", &tokens[*cursor]);
            std::process::exit(0);
        }
    };

    //Basic number variable declaration test
    expect(
        TokenType::DeclarationKeyword {
            keyword: DeclarationKeywords::Let,
        },
        &mut cursor,
    );
    expect(
        TokenType::Identifier {
            identifier: "".to_string(),
        },
        &mut cursor,
    );
    let identifier = match &tokens[cursor - 1] {
        TokenType::Identifier { identifier } => identifier,
        _ => unreachable!(),
    };
    expect(TokenType::Equal, &mut cursor);
    expect(
        TokenType::NumericLiteral {
            numeric_type: NumericLiteralType::Int,
            value: 0.0,
        },
        &mut cursor,
    );

    //todo: for cursor in tokens { match token[cursor] { let, fun, etc => each parser }

    let ast = AbstractSyntaxTree { program: vec![] };
    for i in ast.program {
        println!("{:?}", i);
    }
    println!("Parsing finished successfully")
}
fn parse_expression() {}

struct AbstractSyntaxTree {
    pub program: Vec<Node>,
}
impl AbstractSyntaxTree {}

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    PreIncrement,
    PostIncrement,
    PreDecrement,
    PostDecrement,
    Negation,
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
    Expression,
}
