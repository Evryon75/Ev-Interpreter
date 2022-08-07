use crate::ast::{AbstractSyntaxTree, ExpressionType, Node, Parameter};
use colour::*;
use std::collections::HashMap;
use std::env::var;

pub(crate) fn walk(ast: AbstractSyntaxTree) {
    for i in &ast.program {
        yellow_ln!("{:#?}", i);
    }
    green_ln!("Walking the Abstract Syntax Tree...");

    let mut variables: Vec<HashMap<String, Expression>> = vec![];
    let mut functions: Vec<HashMap<String, Function>> = vec![];

    fn walk_block(
        block: &Vec<Node>,
        variables: &mut Vec<HashMap<String, Expression>>,
        functions: &mut Vec<HashMap<String, Function>>,
    ) {
        (*variables).push(HashMap::new());
        (*functions).push(HashMap::new());
        for node in block {
            let last = variables.len() - 1;
            match node {
                Node::Return { value } => {}
                Node::If {
                    condition,
                    block,
                    else_block,
                } => {}
                Node::While { condition, block } => {}
                Node::VariableDeclaration { value, identifier } => {
                    (*variables)[last].insert(
                        identifier.to_string(),
                        Expression {
                            value: walk_expression(value.to_owned()),
                        },
                    );
                }
                Node::FunctionDeclaration {
                    parameters,
                    identifier,
                    block,
                } => {
                    (*functions)[last].insert(
                        identifier.to_string(),
                        Function {
                            params: parameters.to_owned(),
                            block: block.to_owned(),
                        },
                    );
                }
                Node::VariableReassignment {
                    identifier,
                    new_value,
                } => {}
                Node::ProcedureCall { identifier, params } => {}
            }
        }
        cyan_ln!("////////////////////////////////////////////////////////////////");
        cyan_ln!("\\\\\\\\VARIABLES\\\\\\\\");
        for i in variables.to_vec() {
            cyan_ln!("{:#?}", i)
        }
        cyan_ln!("\\\\\\\\FUNCTIONS\\\\\\\\");
        for i in functions.to_vec() {
            cyan_ln!("{:#?}", i)
        }
        (*variables).remove((*variables).len() - 1);
        (*functions).remove((*functions).len() - 1);
    }

    fn walk_expression(expr: ExpressionType) -> VarType {
        //todo: walk expression, returns false for now
        VarType::BoolExpr { value: false }
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
    StrExpr { value: String },
    BoolExpr { value: bool },
    NumExpr { value: f32 },
}
