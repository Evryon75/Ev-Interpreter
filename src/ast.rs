use std::ptr::null;
use crate::ast::Node::{Expression, VariableDeclaration};
use crate::lexer::{NumericLiteralType, PrimitiveType, TokenType};

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    for i in &tokens {
        println!("RAW: {:?}", i);
    }

    let mut cursor = 0;
    let mut expect = |token_type: TokenType, cursor: &mut usize| {
        println!("EXPECTING: {:?}", token_type);
        if std::mem::discriminant(&tokens[*cursor]) == std::mem::discriminant(&token_type) {
            *cursor += 1;
        } else {
            println!("Parsing Error: Unexpected Token [{:?}] expected: {:?}", &tokens[*cursor], token_type);
            panic!()
        }
    };

    let mut ast = AbstractSyntaxTree { program: vec![] };

    while cursor < tokens.len() {
        match tokens[cursor] {
            TokenType::Let => {
                expect(TokenType::Let, &mut cursor);
                expect(
                    TokenType::Identifier {
                        identifier: "".to_string(),
                    },
                    &mut cursor,
                );
                let id = match &tokens[cursor - 1] {
                    TokenType::Identifier { identifier } => identifier,
                    _ => unreachable!(),
                };
                expect(TokenType::Equal, &mut cursor);
                let expression = parse_expression();
                expect(TokenType::Caster, &mut cursor);
                expect(TokenType::Primitive { primitive_type: PrimitiveType::Int }, &mut cursor);
                let typ = match &tokens[cursor - 1] {
                    TokenType::Primitive { primitive_type } => primitive_type.clone(),
                    _ => unreachable!(),
                };
                expect(TokenType::Semicolon, &mut cursor);

                ast.program.push(VariableDeclaration { variable_type: typ, identifier: id.to_string() })
            }
            TokenType::Fun => {
                expect(TokenType::Fun, &mut cursor);
                expect(TokenType::Identifier { identifier: "".to_string() }, &mut cursor);
                expect(TokenType::Equal, &mut cursor);
                expect(TokenType::LParen, &mut cursor);

                let mut params: Vec<Node> = vec![];
                while tokens[cursor] != TokenType::RParen {
                    expect(TokenType::Identifier { identifier: "".to_string() }, &mut cursor);
                    let identifier = match &tokens[cursor - 1] {
                        TokenType::Identifier { identifier } => identifier,
                        _ => unreachable!(),
                    };
                    expect(TokenType::Caster, &mut cursor);
                    expect(TokenType::Primitive { primitive_type: PrimitiveType::Int }, & mut cursor);
                    let param_type = match &tokens[cursor - 1] {
                        TokenType::Primitive { primitive_type } => primitive_type.clone(),
                        _ => unreachable!(),
                    };
                    if tokens[cursor] != TokenType::RParen {
                        expect(TokenType::Comma, &mut cursor);
                    }
                    params.push(Node::Parameter { param_type, param_identifier: identifier.to_string() });
                }

                expect(TokenType::RParen, &mut cursor);
                expect(TokenType::LBrace, &mut cursor);
                expect(TokenType::RBrace, &mut cursor);
                expect(TokenType::Semicolon, &mut cursor);

                ast.program.push(Node::FunctionDeclaration { parameters: params, body: Box::new(Node::Statement) });

            }
            _ => {
                cursor += 1;
            }
        }
    }

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
    Parameter {
        param_type: PrimitiveType,
        param_identifier: String,
    },
    Expression,
    Statement,
    VariableDeclaration {
        variable_type: PrimitiveType,
        identifier: String,
    },
    FunctionDeclaration {
        parameters: Vec<Node>,
        body: Box<Node>
    }
}