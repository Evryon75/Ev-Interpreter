use crate::ast::ExpressionType::UnaryE;
use crate::lexer::{NumericLiteralType, PrimitiveType, TokenType};
use colour::*;

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) {
    let mut cursor = 0;
    let expect = |token_types: Vec<TokenType>, cursor: &mut usize| {
        let mut received = false;
        for token in &token_types {
            if tokens.len().eq(&cursor) {
                red_ln!("Parsing Error: Semicolon Expected");
                panic!()
            }
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
            TokenType::StringLiteral { value, .. } => {
                *cursor += 1;
                ExpressionType::LiteralE {
                    value: Literal::StringL {
                        value: value.to_string(),
                    },
                }
            }
            TokenType::Not => {
                red_ln!(
                    "The Ev programming language does not provide the \"Not\" [!] logical operator"
                );
                blue_ln!("Use this function instead:\nfun not = (param as bool) {\n   if param {\n      return false;\n   } else {\n      return true;\n   }\n}");
                grey_ln!("You can also use a ternary operator: \"true ? false : true\"\nComing soon [maybe]");
                panic!()
            }
            TokenType::BooleanLiteral { value } => {
                *cursor += 1;
                let op = match token_vec[*cursor] {
                    TokenType::SubtractionOp => Operator::Minus,
                    TokenType::AdditionOp => Operator::Minus,
                    TokenType::DivisionOp => Operator::Minus,
                    TokenType::MultiplicationOp => Operator::Minus,
                    TokenType::And => Operator::And,
                    TokenType::Or => Operator::Or,
                    TokenType::Ternary => Operator::Ternary,
                    _ => Operator::None,
                };
                if op == Operator::Minus {
                    red_ln!(
                        "Parsing Error: Cannot use mathematical operator in boolean expression"
                    );
                    panic!()
                }
                if op == Operator::None {
                    ExpressionType::LiteralE {
                        value: Literal::BooleanL {
                            value: value.to_owned(),
                        },
                    }
                } else if op == Operator::Ternary {
                    red_ln!(
                        "Parsing Error: The Ev programming language does not provide the \"Ternary operator\""
                    );
                    grey_ln!("I was going to add it but then i changed my mind");
                    panic!()
                } else {
                    *cursor += 1;
                    ExpressionType::BinaryE {
                        op,
                        lhs: Box::new(ExpressionType::LiteralE {
                            value: Literal::BooleanL {
                                value: value.to_owned(),
                            },
                        }),
                        rhs: Box::new(parse_expression(token_vec, cursor)),
                    }
                }
            }
            TokenType::Identifier { identifier } => {
                *cursor += 1;
                let op = match token_vec[*cursor] {
                    TokenType::SubtractionOp => Operator::Minus,
                    TokenType::AdditionOp => Operator::Plus,
                    TokenType::DivisionOp => Operator::Division,
                    TokenType::MultiplicationOp => Operator::Multiplication,
                    TokenType::And => Operator::Or,
                    TokenType::Or => Operator::Or,
                    _ => Operator::None,
                };
                if op == Operator::Or {
                    red_ln!(
                        "Parsing Error: Cannot use boolean operator in mathematical expression"
                    );
                    panic!()
                }
                if op == Operator::None {
                    ExpressionType::Ident {
                        value: identifier.to_string(),
                    }
                } else {
                    *cursor += 1;
                    ExpressionType::BinaryE {
                        op,
                        lhs: Box::from(ExpressionType::Ident {
                            value: identifier.to_string(),
                        }),
                        rhs: Box::from(parse_expression(token_vec, cursor)),
                    }
                }
            }
            TokenType::SubtractionOp => {
                *cursor += 3;
                if std::mem::discriminant(&token_vec[*cursor - 2])
                    == std::mem::discriminant(&TokenType::NumericLiteral {
                        numeric_type: NumericLiteralType::Int,
                        value: 0.0,
                    })
                {
                    if &token_vec[*cursor - 1] != &TokenType::RParen {
                        ExpressionType::BinaryE {
                            op: match &token_vec[*cursor - 1] {
                                TokenType::SubtractionOp => Operator::Minus,
                                TokenType::AdditionOp => Operator::Plus,
                                TokenType::DivisionOp => Operator::Division,
                                TokenType::MultiplicationOp => Operator::Multiplication,
                                _ => {
                                    unreachable!()
                                }
                            },
                            lhs: Box::new(UnaryE {
                                op: Operator::Minus,
                                child: Box::new(ExpressionType::LiteralE {
                                    value: Literal::NumberL {
                                        value: *match &token_vec[*cursor - 2] {
                                            TokenType::NumericLiteral { value, .. } => value,
                                            _ => {
                                                unreachable!()
                                            }
                                        },
                                    },
                                }),
                            }),
                            rhs: Box::new(parse_expression(token_vec, cursor)),
                        }
                    } else {
                        UnaryE {
                            op: Operator::Minus,
                            child: Box::new(ExpressionType::LiteralE {
                                value: Literal::NumberL {
                                    value: *match &token_vec[*cursor - 2] {
                                        TokenType::NumericLiteral { value, .. } => value,
                                        _ => {
                                            unreachable!()
                                        }
                                    },
                                },
                            }),
                        }
                    }
                } else {
                    red_ln!("Parsing Error: Unary Operator on non Numeric Literal");
                    panic!()
                }
            }
            TokenType::LParen => {
                *cursor += 1;
                let result: ExpressionType = parse_expression(token_vec, cursor);
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
                        ExpressionType::BinaryE {
                            op: oper,
                            lhs: Box::from(result),
                            rhs: Box::from(parse_expression(token_vec, cursor)),
                        }
                    } else {
                        result
                    }
                } else {
                    red_ln!("Parsing Error: Opening Parenthesis without Closing Parenthesis");
                    blue_ln!("Note: The developer was too lazy to fix this bug (i tried i swear) so if you are trying to put something like \"(-1)\" in an expression you have to change it to \"(-1 + 0)\"");
                    panic!()
                }
            }
            TokenType::NumericLiteral { value, .. } => {
                let mut lhs: ExpressionType = ExpressionType::LiteralE {
                    value: Literal::NumberL { value: *value },
                };
                let mut oper = Operator::None;
                *cursor += 1;
                lhs = if vec![
                    TokenType::DivisionOp,
                    TokenType::MultiplicationOp,
                    TokenType::GreaterThan,
                    TokenType::LessThan,
                    TokenType::DoubleEqual,
                ]
                .contains(&token_vec[*cursor])
                {
                    oper = match &token_vec[*cursor] {
                        TokenType::DivisionOp => Operator::Division,
                        TokenType::MultiplicationOp => Operator::Multiplication,
                        TokenType::GreaterThan => Operator::Greater,
                        TokenType::LessThan => Operator::Less,
                        TokenType::DoubleEqual => Operator::DoubleEqual,
                        _ => Operator::None,
                    };
                    *cursor += 1;
                    ExpressionType::BinaryE {
                        op: oper,
                        lhs: Box::new(lhs),
                        rhs: Box::new(parse_expression(token_vec, cursor)),
                    }
                } else {
                    lhs
                };
                let rhs: ExpressionType = if vec![
                    TokenType::DivisionOp,
                    TokenType::MultiplicationOp,
                    TokenType::GreaterThan,
                    TokenType::LessThan,
                    TokenType::DoubleEqual,
                ]
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
                if id == "output" || id == "input" {
                    red_ln!("Cannot overwrite I/O functions [input(), output()]");
                    panic!();
                }
                expect(vec![TokenType::Equal], &mut cursor);
                let expression = parse_expression(&tokens, &mut cursor);
                expect(vec![TokenType::Semicolon], &mut cursor);

                ast.program.push(Node::VariableDeclaration {
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
                if id == "output" || id == "input" {
                    red_ln!("Cannot overwrite I/O functions [input(), output()]");
                    panic!();
                }
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
    Ternary,
    Plus,
    Minus,
    Multiplication,
    Division,
    And,
    Or,
    Greater,
    Less,
    DoubleEqual,
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
#[derive(Debug, PartialEq, Clone)]
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
    Ident {
        value: String,
    },
    None,
}
#[derive(Debug, PartialEq, Clone)]
enum Literal {
    NumberL { value: f64 },
    StringL { value: String },
    BooleanL { value: bool },
}
