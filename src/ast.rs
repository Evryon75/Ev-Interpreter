use crate::ast::Node::{Expression, VariableDeclaration};
use crate::lexer::{NumericLiteralType, PrimitiveType, TokenType};
use colour::*;
use std::ptr::null;

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    let mut cursor = 0;
    let mut expect = |token_types: Vec<TokenType>, cursor: &mut usize| {
        let mut received = false;
        for token in &token_types {
            magenta_ln!("EXPECTING: {:?}", &token);
            if std::mem::discriminant(&tokens[*cursor]) == std::mem::discriminant(&token) {
                received = true;
            }
            if received {
                *cursor += 1;
            } else {
                red_ln!(
                    "Parsing Error: Unexpected Token [{:?}] expected: {:?}",
                    &tokens[*cursor],
                    token_types
                );
                panic!()
            }
        }
    };
    fn parse_expression(token_vec: &Vec<TokenType>, cursor: &mut usize) -> ExpressionType {
        match &token_vec[*cursor] {
            TokenType::LParen => {
                *cursor += 1;
                let mut result: ExpressionType = parse_expression(token_vec, cursor);
                *cursor += 1;
                if token_vec[*cursor - 1] == TokenType::RParen {
                    if vec![
                        TokenType::DivisionOp,
                        TokenType::MultiplicationOp,
                        TokenType::AdditionOp,
                        TokenType::SubtractionOp,
                    ]
                    .contains(&token_vec[*cursor])
                    {
                        let oper = match &token_vec[*cursor] {
                            TokenType::SubtractionOp => Operator::Minus,
                            TokenType::AdditionOp => Operator::Plus,
                            TokenType::DivisionOp => Operator::Division,
                            TokenType::MultiplicationOp => Operator::Multiplication,
                            _ => unreachable!(),
                        };
                        *cursor += 1;
                        result = ExpressionType::BinaryE {
                            op: oper,
                            lhs: Box::from(result),
                            rhs: Box::from(parse_expression(token_vec, cursor))
                        }
                    }
                } else {
                    red_ln!("Parsing Error: Opening Parenthesis without Closing Parenthesis");
                    panic!()
                };
                result
            }
            TokenType::NumericLiteral { value, .. } => {
                let mut lhs: ExpressionType = ExpressionType::LiteralE {
                    value: Literal::NumberL { value: *value },
                };

                let mut oper = Operator::None;
                *cursor += 1;
                lhs = if vec![TokenType::DivisionOp, TokenType::MultiplicationOp]
                    .contains(&token_vec[*cursor])
                {
                    oper = match &token_vec[*cursor] {
                        TokenType::DivisionOp => Operator::Division,
                        TokenType::MultiplicationOp => Operator::Multiplication,
                        _ => Operator::None,
                    };
                    *cursor += 1;
                    //todo: Operator precedence
                    ExpressionType::BinaryE {
                        op: oper,
                        lhs: Box::new(lhs),
                        rhs: Box::new(parse_expression(token_vec, cursor)),
                    }
                } else {
                    lhs
                };
                let rhs: ExpressionType = if vec![TokenType::AdditionOp, TokenType::SubtractionOp]
                    .contains(&token_vec[*cursor])
                {
                    oper = match &token_vec[*cursor] {
                        TokenType::SubtractionOp => Operator::Minus,
                        TokenType::AdditionOp => Operator::Plus,
                        _ => Operator::None,
                    };
                    *cursor += 1;
                    parse_expression(token_vec, cursor)
                } else {
                    ExpressionType::None
                };

                if rhs == ExpressionType::None {
                    lhs
                } else {
                    ExpressionType::BinaryE {
                        op: oper,
                        lhs: Box::from(lhs),
                        rhs: Box::from(rhs),
                    }
                }
            }
            _ => {
                red_ln!(
                    "Parsing Error: Invalid Expression at [{:?}]",
                    token_vec[*cursor]
                );
                panic!();
            }
        }
    }

    let mut ast = AbstractSyntaxTree { program: vec![] };

    while cursor < tokens.len() {
        match tokens[cursor] {
            TokenType::Let => {
                expect(vec![TokenType::Let], &mut cursor);
                expect(
                    vec![TokenType::Identifier {
                        identifier: "".to_string(),
                    }],
                    &mut cursor,
                );
                let id = match &tokens[cursor - 1] {
                    TokenType::Identifier { identifier } => identifier,
                    _ => unreachable!(),
                };
                expect(vec![TokenType::Equal], &mut cursor);
                let expression = parse_expression(&tokens, &mut cursor);
                expect(vec![TokenType::Semicolon], &mut cursor);

                ast.program.push(VariableDeclaration {
                    value: expression,
                    identifier: id.to_string(),
                })
            }
            TokenType::Fun => {
                expect(vec![TokenType::Fun], &mut cursor);
                expect(
                    vec![TokenType::Identifier {
                        identifier: "".to_string(),
                    }],
                    &mut cursor,
                );
                let id = match &tokens[cursor - 1] {
                    TokenType::Identifier { identifier } => identifier,
                    _ => unreachable!(),
                };
                expect(vec![TokenType::Equal], &mut cursor);
                expect(vec![TokenType::LParen], &mut cursor);

                let mut params: Vec<Node> = vec![];
                while tokens[cursor] != TokenType::RParen {
                    expect(
                        vec![TokenType::Identifier {
                            identifier: "".to_string(),
                        }],
                        &mut cursor,
                    );
                    let identifier = match &tokens[cursor - 1] {
                        TokenType::Identifier { identifier } => identifier,
                        _ => unreachable!(),
                    };
                    expect(vec![TokenType::Caster], &mut cursor);
                    expect(
                        vec![TokenType::Primitive {
                            primitive_type: PrimitiveType::Int,
                        }],
                        &mut cursor,
                    );
                    let param_type = match &tokens[cursor - 1] {
                        TokenType::Primitive { primitive_type } => primitive_type.clone(),
                        _ => unreachable!(),
                    };
                    if tokens[cursor] != TokenType::RParen {
                        expect(vec![TokenType::Comma], &mut cursor);
                    }
                    params.push(Node::Parameter {
                        param_type,
                        param_identifier: identifier.to_string(),
                    });
                }

                expect(vec![TokenType::RParen], &mut cursor);
                expect(vec![TokenType::LBrace], &mut cursor);
                expect(vec![TokenType::RBrace], &mut cursor);
                expect(vec![TokenType::Semicolon], &mut cursor);

                ast.program.push(Node::FunctionDeclaration {
                    parameters: params,
                    identifier: id.to_string(),

                    body: Box::new(Node::Statement),
                });
            }
            _ => {
                println!("Parsing Error: Unexpected Token [{:?}]", &tokens[cursor]);
                panic!()
            }
        }
    }

    for i in ast.program {
        yellow_ln!("{:#?}", i);
    }
    green_ln!("Parsing finished successfully")
}

struct AbstractSyntaxTree {
    pub program: Vec<Node>,
}
impl AbstractSyntaxTree {}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    Negation,
    None,
}
#[derive(Debug, PartialEq)]
enum Node {
    Parameter {
        param_type: PrimitiveType,
        param_identifier: String,
    },
    Expression {
        value: ExpressionType,
    },
    Statement,
    VariableDeclaration {
        value: ExpressionType,
        identifier: String,
    },
    FunctionDeclaration {
        parameters: Vec<Node>,
        identifier: String,
        body: Box<Node>,
    },
}
#[derive(Debug, PartialEq)]
enum ExpressionType {
    LiteralE {
        value: Literal,
    },
    BinaryE {
        op: Operator,
        lhs: Box<ExpressionType>,
        rhs: Box<ExpressionType>,
    },
    UnaryE {
        op: Operator,
        child: Box<ExpressionType>,
    },
    GroupingE {
        value: Box<ExpressionType>,
    },
    Ident {
        value: String,
    },
    None
}
#[derive(Debug, PartialEq)]
enum Literal {
    NumberL { value: f64 },
    StringL { value: String },
    BooleanL { value: bool },
}
