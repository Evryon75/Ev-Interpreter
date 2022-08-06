use crate::ast::AbstractSyntaxTree;
use colour::*;

pub(crate) fn walk(ast: AbstractSyntaxTree) {
    for i in ast.program {
        yellow_ln!("{:#?}", i);
    }
    green_ln!("Walking the Abstract Syntax Tree...");
}
