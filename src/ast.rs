use crate::ast::ExpressionType::UnaryE;
use crate::lexer::{NumericLiteralType, PrimitiveType, TokenType};
use colour::*;

pub(crate) fn parse_tokens(tokens: Vec<TokenType>) -> AbstractSyntaxTree {
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
    };
    fn parse_expression(token_vec: &Vec<TokenType>, cursor: &mut usize) -> ExpressionType {
        let expect_expr = |token_types: Vec<TokenType>, cursor: &mut usize| {
            let mut received = false;
            for tok in &token_types {
                if token_vec.len().eq(&cursor) {
                    red_ln!("Parsing Error: Semicolon Expected");
                    panic!()
                }
                if std::mem::discriminant(&token_vec[*cursor]) == std::mem::discriminant(&tok) {
                    received = true;
                }
            }
            if received {
                *cursor += 1;
            } else {
                red_ln!(
                    "Parsing Error: Unexpected Token [{:?}] expected: {:?}",
                    &token_vec[*cursor],
                    token_types
                );
                blue_ln!("NOTE: Only literal values, identifiers, and function calls are allowed as parameters, use a temp variable instead");
                panic!()
            }
        };
        match &token_vec[*cursor] {
            TokenType::StringLiteral { value, .. } => {
                *cursor += 1;
                ExpressionType::LiteralE {
                    value: Literal::StringL {
                        value: value.to_string(),
                    },
                }
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
                    TokenType::GreaterThan => Operator::GreaterThan,
                    TokenType::LessThan => Operator::LessThan,
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
                    TokenType::DoubleEqual => Operator::DoubleEqual,
                    TokenType::And => Operator::And,
                    TokenType::Or => Operator::Or,
                    TokenType::GreaterThan => Operator::GreaterThan,
                    TokenType::LessThan => Operator::LessThan,
                    _ => Operator::None,
                };
                if op == Operator::None && token_vec[*cursor] != TokenType::LParen {
                    ExpressionType::Ident {
                        value: identifier.to_string(),
                    }
                } else if token_vec[*cursor] == TokenType::LParen {
                    *cursor += 1;
                    let mut params_vec: Vec<ExpressionType> = vec![];

                    if token_vec[*cursor] != TokenType::RParen {
                        loop {
                            params_vec.push(parse_expression(token_vec, cursor));
                            if token_vec[*cursor] != TokenType::RParen {
                                expect_expr(vec![TokenType::Comma], cursor);
                            } else {
                                expect_expr(vec![TokenType::RParen], cursor);
                                break;
                            }
                        }
                    } else {
                        expect_expr(vec![TokenType::RParen], cursor);
                    }

                    ExpressionType::FunctionCall {
                        identifier: identifier.to_string(),
                        params: params_vec,
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
                        TokenType::And,
                        TokenType::Or,
                        TokenType::DoubleEqual,
                    ]
                    .contains(&token_vec[*cursor])
                    {
                        let oper = match &token_vec[*cursor] {
                            TokenType::SubtractionOp => Operator::Minus,
                            TokenType::AdditionOp => Operator::Plus,
                            TokenType::DivisionOp => Operator::Division,
                            TokenType::MultiplicationOp => Operator::Multiplication,
                            TokenType::And => Operator::And,
                            TokenType::Or => Operator::Or,
                            TokenType::DoubleEqual => Operator::DoubleEqual,
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
                let lhs: ExpressionType = ExpressionType::LiteralE {
                    value: Literal::NumberL { value: *value },
                };
                *cursor += 1;
                if vec![
                    TokenType::DivisionOp,
                    TokenType::MultiplicationOp,
                    TokenType::AdditionOp,
                    TokenType::SubtractionOp,
                    TokenType::GreaterThan,
                    TokenType::LessThan,
                    TokenType::DoubleEqual,
                ]
                .contains(&token_vec[*cursor])
                {
                    let oper = match &token_vec[*cursor] {
                        TokenType::DivisionOp => Operator::Division,
                        TokenType::MultiplicationOp => Operator::Multiplication,
                        TokenType::AdditionOp => Operator::Plus,
                        TokenType::SubtractionOp => Operator::Minus,
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

    let mut ast = AbstractSyntaxTree {
        program: vec![],
        global_scope: vec![],
    };

    fn parse_statement<F: Fn(Vec<TokenType>, &mut usize) + Copy>(
        tokens: &Vec<TokenType>,
        mut cursor: &mut usize,
        expect: F,
    ) -> Node {
        let expect_expr = |token_types: Vec<TokenType>, cursor: &mut usize| {
            let mut received = false;
            for tok in &token_types {
                if tokens.len().eq(&cursor) {
                    red_ln!("Parsing Error: Semicolon Expected");
                    panic!()
                }
                if std::mem::discriminant(&tokens[*cursor]) == std::mem::discriminant(&tok) {
                    received = true;
                }
            }
            if received {
                *cursor += 1;
            } else {
                red_ln!(
                    "Parsing Error: Unexpected Token [{:?}] expected: {:?}",
                    &tokens[*cursor],
                    token_types
                );
                blue_ln!("NOTE: Only literal values, identifiers, and function calls are allowed as parameters, use a temp variable instead");
                panic!()
            }
        };
        match &tokens[*cursor] {
            TokenType::Identifier { identifier } => {
                *cursor += 1;
                expect(vec![TokenType::Equal, TokenType::LParen], &mut cursor);
                if tokens[*cursor - 1] == TokenType::Equal {
                    let new_value = parse_expression(&tokens, &mut cursor);
                    expect(vec![TokenType::Semicolon], &mut cursor);

                    Node::VariableReassignment {
                        identifier: identifier.to_string(),
                        new_value,
                    }
                } else if tokens[*cursor - 1] == TokenType::LParen {
                    let mut params: Vec<ExpressionType> = vec![];

                    if tokens[*cursor] != TokenType::RParen {
                        loop {
                            params.push(parse_expression(&tokens, cursor));
                            if tokens[*cursor] != TokenType::RParen {
                                expect_expr(vec![TokenType::Comma], cursor);
                            } else {
                                expect_expr(vec![TokenType::RParen], cursor);
                                break;
                            }
                        }
                    } else {
                        *cursor += 1;
                    }
                    expect(vec![TokenType::Semicolon], &mut cursor);
                    Node::ProcedureCall {
                        identifier: identifier.to_string(),
                        params,
                    }
                } else {
                    red_ln!("Parsing error: Expected \"=\" or \"(\"");
                    panic!();
                }
            }
            TokenType::Return => {
                expect(vec![TokenType::Return], &mut cursor);
                let res = parse_expression(&tokens, &mut cursor);
                expect(vec![TokenType::Semicolon], &mut cursor);
                Node::Return { value: res }
            }
            TokenType::If => {
                expect(vec![TokenType::If], &mut cursor);
                let condition = parse_expression(&tokens, &mut cursor);
                expect(vec![TokenType::LBrace], &mut cursor);
                let mut block: Vec<Node> = vec![];
                while tokens[*cursor] != TokenType::RBrace {
                    block.push(parse_statement(tokens, cursor, expect));
                }
                expect(vec![TokenType::RBrace], &mut cursor);
                let mut else_block: Vec<Node> = vec![];

                if &tokens[*cursor] == &TokenType::Else {
                    expect(vec![TokenType::Else], &mut cursor);
                    expect(vec![TokenType::LBrace], &mut cursor);
                    while tokens[*cursor] != TokenType::RBrace {
                        else_block.push(parse_statement(tokens, cursor, expect));
                    }
                    expect(vec![TokenType::RBrace], &mut cursor);
                }
                expect(vec![TokenType::Semicolon], &mut cursor);
                Node::If {
                    condition,
                    block,
                    else_block,
                }
            }
            TokenType::While => {
                expect(vec![TokenType::While], &mut cursor);
                let condition = parse_expression(&tokens, &mut cursor);
                expect(vec![TokenType::LBrace], &mut cursor);
                let mut block: Vec<Node> = vec![];
                while tokens[*cursor] != TokenType::RBrace {
                    block.push(parse_statement(tokens, cursor, expect));
                }
                expect(vec![TokenType::RBrace], &mut cursor);

                Node::While { condition, block }
            }
            TokenType::Let => {
                expect(vec![TokenType::Let], &mut cursor);
                expect(
                    vec![TokenType::Identifier {
                        identifier: "".to_string(),
                    }],
                    &mut cursor,
                );
                let id = match &tokens[*cursor - 1] {
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

                Node::VariableDeclaration {
                    value: expression,
                    identifier: id.to_string(),
                }
            }
            TokenType::Fun => {
                expect(vec![TokenType::Fun], &mut cursor);
                expect(
                    vec![TokenType::Identifier {
                        identifier: "".to_string(),
                    }],
                    &mut cursor,
                );
                let id = match &tokens[*cursor - 1] {
                    TokenType::Identifier { identifier } => identifier,
                    _ => unreachable!(),
                };
                if id == "output" || id == "input" {
                    red_ln!("Cannot overwrite I/O functions [input(), output()]");
                    panic!();
                }
                expect(vec![TokenType::Equal], &mut cursor);
                expect(vec![TokenType::LParen], &mut cursor);

                let mut params: Vec<Parameter> = vec![];
                while tokens[*cursor] != TokenType::RParen {
                    expect(
                        vec![TokenType::Identifier {
                            identifier: "".to_string(),
                        }],
                        &mut cursor,
                    );
                    let identifier = match &tokens[*cursor - 1] {
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
                    let param_type = match &tokens[*cursor - 1] {
                        TokenType::Primitive { primitive_type } => primitive_type.clone(),
                        _ => unreachable!(),
                    };
                    if tokens[*cursor] != TokenType::RParen {
                        expect(vec![TokenType::Comma], &mut cursor);
                    }
                    params.push(Parameter {
                        param_type,
                        param_identifier: identifier.to_string(),
                    });
                }

                expect(vec![TokenType::RParen], &mut cursor);
                expect(vec![TokenType::LBrace], &mut cursor);
                let mut block: Vec<Node> = vec![];
                while tokens[*cursor] != TokenType::RBrace {
                    block.push(parse_statement(tokens, cursor, expect));
                }
                expect(vec![TokenType::RBrace], &mut cursor);
                expect(vec![TokenType::Semicolon], &mut cursor);

                Node::FunctionDeclaration {
                    parameters: params,
                    identifier: id.to_string(),
                    block,
                }
            }
            _ => {
                red_ln!("Parsing Error: Unexpected Token [{:?}]", &tokens[*cursor]);
                panic!()
            }
        }
    };

    while cursor < tokens.len() {
        ast.program
            .push(parse_statement(&tokens, &mut cursor, expect));
    }

    green_ln!("Parsing: finished successfully ✔");
    ast
}
#[derive(Debug, PartialEq, Clone)]
pub struct AbstractSyntaxTree {
    pub(crate) program: Vec<Node>,
    pub(crate) global_scope: Vec<Node>,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    GreaterThan,
    LessThan,
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
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    param_type: PrimitiveType,
    param_identifier: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Return {
        value: ExpressionType,
    },
    If {
        condition: ExpressionType,
        block: Vec<Node>,
        else_block: Vec<Node>,
    },
    While {
        condition: ExpressionType,
        block: Vec<Node>,
    },
    VariableDeclaration {
        value: ExpressionType,
        identifier: String,
    },
    FunctionDeclaration {
        parameters: Vec<Parameter>,
        identifier: String,
        block: Vec<Node>,
    },
    VariableReassignment {
        identifier: String,
        new_value: ExpressionType,
    },
    ProcedureCall {
        identifier: String,
        params: Vec<ExpressionType>,
    },
}
#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionType {
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
    FunctionCall {
        identifier: String,
        params: Vec<ExpressionType>,
    },
}
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    NumberL { value: f64 },
    StringL { value: String },
    BooleanL { value: bool },
}
