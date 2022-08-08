use crate::ast::{AbstractSyntaxTree, ExpressionType, Literal, Node, Parameter};
use colour::*;
use std::collections::HashMap;
use std::env::var;

pub(crate) fn walk(ast: AbstractSyntaxTree) {

    let mut variables: Vec<HashMap<String, Expression>> = vec![];
    let mut functions: Vec<HashMap<String, Function>> = vec![];

    fn walk_block(
        block: &Vec<Node>,
        variables: &mut Vec<HashMap<String, Expression>>,
        functions: &mut Vec<HashMap<String, Function>>,
    ) -> VarType {
        (*variables).push(HashMap::new());
        (*functions).push(HashMap::new());
        let mut function_result = VarType::None;
        for node in block {
            let last = (&variables).len() - 1;
            match node {
                Node::Return { value } => {
                    function_result = walk_expression(value.to_owned());;
                }
                Node::If {
                    condition,
                    block,
                    else_block,
                } => {
                    let cond: bool = match walk_expression(condition.to_owned()) {
                        VarType::StrExpr { .. } => {
                            red_ln!("Expected boolean expression, found string expression");
                            panic!()
                        }
                        VarType::BoolExpr { value } => {
                            value
                        }
                        VarType::NumExpr { .. } => {
                            red_ln!("Expected boolean expression, found mathematical expression");
                            panic!()
                        }
                        _ => {
                            red_ln!("Expected boolean expression, found null (make sure the function has a return statement)");
                            panic!()}
                    };
                    if cond {
                        walk_block(block, variables, functions);
                    } else {
                        walk_block(else_block, variables, functions);
                    }
                }
                Node::While { condition, block } => {
                    let cond: bool = match walk_expression(condition.to_owned()) {
                        VarType::StrExpr { .. } => {
                            red_ln!("Expected boolean expression, found string expression");
                            panic!()
                        }
                        VarType::BoolExpr { value } => {
                            value
                        }
                        VarType::NumExpr { .. } => {
                            red_ln!("Expected boolean expression, found mathematical expression");
                            panic!()
                        }
                        _ => {
                            red_ln!("Expected boolean expression, found null (make sure the function has a return statement)");
                            panic!()
                        }
                    };
                    while cond {
                        walk_block(block, variables, functions);
                    }
                }
                Node::VariableDeclaration { value, identifier } => {
                    let mut duplicate = false;
                    for i in variables.clone() {
                        if i.contains_key(identifier) {
                            duplicate = true;
                        }
                    }
                    if !duplicate {
                        (*variables)[last].insert(
                            identifier.to_string(),
                            Expression {
                                value: walk_expression(value.to_owned()),
                            },
                        );
                    } else {
                        red_ln!("Walking error: duplicate variable identifier [{}]", identifier);
                        panic!()
                    }
                }
                Node::FunctionDeclaration {
                    parameters,
                    identifier,
                    block,
                } => {
                    let mut duplicate = false;
                    for i in functions.clone() {
                        if i.contains_key(identifier) {
                            duplicate = true;
                        }
                    }
                    if !duplicate {
                        (*functions)[last].insert(
                            identifier.to_string(),
                            Function {
                                params: parameters.to_owned(),
                                block: block.to_owned(),
                            },
                        );
                    } else {
                        red_ln!("Walking error: duplicate function identifier [{}]", identifier);
                        panic!()
                    }
                }
                Node::VariableReassignment {
                    identifier,
                    new_value,
                } => {
                    let mut found = false;
                    for i in &mut *variables {
                        if i.contains_key(identifier) {
                            i.insert(
                                identifier.to_string(),
                                Expression {
                                    value: walk_expression(new_value.to_owned()),
                                },
                            );
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        red_ln!(
                            "Walking Error: Identifier [{:?}] not found in this scope",
                            identifier
                        );
                        grey_ln!("Variables available in this scope:");
                        for i in &mut *variables {
                            grey!("{:?}", i.keys());
                        }
                        println!(); //Get a new line
                        panic!()
                    }
                }
                Node::ProcedureCall { identifier, params } => {
                    match &*identifier.as_str() {
                        "output" => {
                            print(params);
                        }
                        "show_scopes" => {
                            cyan_ln!("v [Variables]");
                            for i in variables.to_vec() {
                                cyan_ln!("{:#?}", i);
                            }
                            cyan_ln!("v [Functions]");
                            for i in functions.to_vec() {
                                cyan_ln!("{:#?}", i);
                            }
                            if params.len() > 0 {
                                grey_ln!(
                                    "No parameters needed, {} parameter{} found",
                                    params.len(),
                                    if params.len() == 1 { "" } else { "s" }
                                );
                            }
                        }
                        _ => {}
                    }
                    println!(); //Getting a new line
                }
            }

        }
        (*variables).remove((*variables).len() - 1);
        (*functions).remove((*functions).len() - 1);
        function_result
    }

    fn walk_expression(expr: ExpressionType) -> VarType {
        //todo: walk expression, returns false for now
        VarType::BoolExpr { value: false }
    }
    fn print(params: &Vec<ExpressionType>) {
        for i in params {
            let result: String = match i {
                ExpressionType::LiteralE { value } => match value {
                    Literal::NumberL { value } => value.to_string(),
                    Literal::StringL { value } => value.to_string(),
                    Literal::BooleanL { value } => value.to_string(),
                },
                ExpressionType::BinaryE { .. } => "NOT IMPLEMENTED".to_string(),
                ExpressionType::UnaryE { .. } => "NOT IMPLEMENTED".to_string(),
                ExpressionType::Ident { .. } => "NOT IMPLEMENTED".to_string(),
                ExpressionType::FunctionCall { .. } => {
                    "NOT IMPLEMENTED".to_string()
                }
            };
            print!("{} ", result);
        }
    }

    walk_block(&ast.program, &mut variables, &mut functions);
}

#[derive(Debug, Clone)]
struct Function {
    params: Vec<Parameter>,
    block: Vec<Node>,
}
#[derive(Debug, Clone)]
struct Expression {
    value: VarType,
}
#[derive(Debug, Clone)]
enum VarType {
    None,
    StrExpr { value: String },
    BoolExpr { value: bool },
    NumExpr { value: f32 },
}
