use crate::ast::{AbstractSyntaxTree, ExpressionType, Literal, Node, Operator, Parameter};
use colour::*;
use std::collections::HashMap;
use std::io::stdin;

//STEP THREE: Walking
pub(crate) fn walk(ast: AbstractSyntaxTree) {
    //Memory
    let mut variables: Vec<HashMap<String, Expression>> = vec![];
    let mut functions: Vec<HashMap<String, Function>> = vec![];

    fn walk_block(
        //Recursively asking for parameters
        block: &Vec<Node>,
        mut variables: &mut Vec<HashMap<String, Expression>>,
        mut functions: &mut Vec<HashMap<String, Function>>,
        params_passed: &Vec<ExpressionType>,
        params_asked: &Vec<Parameter>,
    ) -> VarType {
        //When entering a new block, a new scope is added to both vectors, any new variables or functions will be added to the last scope
        (*variables).push(HashMap::new());
        (*functions).push(HashMap::new());

        //Save the parameters to the function's variables
        let mut walked_params = vec![];
        for i in params_passed {
            walked_params.push(walk_expression(i, &mut variables, &mut functions));
        }
        if params_asked.len() == params_passed.len() {
            let mut index = 0;
            while index < params_asked.len() {
                let las = (*variables).len() - 1;
                (*variables)[las].insert(
                    (*params_asked[index].param_identifier).to_string(),
                    Expression {
                        value: walked_params[index].to_owned(),
                    },
                );
                index += 1;
            }
        } else {
            red_ln!("Walking error: the number of given parameters must be equal to the number of parameters in the function declaration");
            panic!();
        }
        //For returning values from functions
        let mut function_result = VarType::None;
        let mut returned = false;
        for node in block {
            if returned {
                break;
            }
            let last = (&variables).len() - 1;
            match node {
                //Do something based on the matched node and its provided information
                Node::Return { value } => {
                    function_result =
                        walk_expression(&value.to_owned(), &mut variables, &mut functions);
                    returned = true;
                }
                Node::If {
                    condition,
                    block,
                    else_block,
                } => {
                    let cond: bool = match walk_expression(
                        &condition.to_owned(),
                        variables,
                        functions,
                    ) {
                        VarType::StrExpr { .. } => {
                            red_ln!("Expected boolean expression, found string expression");
                            panic!()
                        }
                        VarType::BoolExpr { value } => value,
                        VarType::NumExpr { .. } => {
                            red_ln!("Expected boolean expression, found mathematical expression");
                            panic!()
                        }
                        _ => {
                            red_ln!("Expected boolean expression, found null (make sure the function has a return statement)");
                            panic!()
                        }
                    };
                    if cond {
                        walk_block(block, variables, functions, &vec![], &vec![]);
                    } else {
                        walk_block(else_block, variables, functions, &vec![], &vec![]);
                    }
                }
                Node::While { condition, block } => {
                    let mut cond: bool = match walk_expression(
                        &condition.to_owned(),
                        variables,
                        functions,
                    ) {
                        VarType::StrExpr { .. } => {
                            red_ln!("Expected boolean expression, found string expression");
                            panic!()
                        }
                        VarType::BoolExpr { value } => value,
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
                        walk_block(block, variables, functions, &vec![], &vec![]);
                        cond = match walk_expression(&condition.to_owned(), variables, functions) {
                            VarType::StrExpr { .. } => {
                                red_ln!("Expected boolean expression, found string expression");
                                panic!()
                            }
                            VarType::BoolExpr { value } => value,
                            VarType::NumExpr { .. } => {
                                red_ln!(
                                    "Expected boolean expression, found mathematical expression"
                                );
                                panic!()
                            }
                            _ => {
                                red_ln!("Expected boolean expression, found null (make sure the function has a return statement)");
                                panic!()
                            }
                        };
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
                        let val = Expression {
                            value: walk_expression(&value.to_owned(), variables, functions),
                        };
                        (*variables)[last].insert(identifier.to_string(), val);
                    } else {
                        red_ln!(
                            "Walking error: duplicate variable identifier [{}]",
                            identifier
                        );
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
                        red_ln!(
                            "Walking error: duplicate function identifier [{}]",
                            identifier
                        );
                        panic!()
                    }
                }
                Node::VariableReassignment {
                    identifier,
                    new_value,
                } => {
                    let mut found = false;
                    let val = Expression {
                        value: walk_expression(&new_value.to_owned(), variables, functions),
                    };
                    for i in &mut *variables {
                        if i.contains_key(identifier) {
                            i.insert(identifier.to_string(), val);
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
                //This arm has hard coded builtin functions
                Node::ProcedureCall { identifier, params } => {
                    match &*identifier.as_str() {
                        "output" => {
                            print(params, variables, functions);
                            println!(); //Getting a new line
                        }
                        "free" => {
                            for i in params {
                                let mut t = 0;
                                while t < variables.len() {
                                    if variables[t].contains_key(match i {
                                        ExpressionType::Ident { value } => {
                                            value
                                        }
                                        _ => {
                                            red_ln!("Walking error: cannot free anything that is not an identifier");
                                            panic!();
                                        }
                                    }) {
                                        variables[t].remove(match i {
                                            ExpressionType::Ident { value } => {
                                                value
                                            }
                                            _ => unreachable!()
                                        });
                                    }
                                    t += 1;
                                }
                                t = 0;
                                while t < functions.len() {
                                    if functions[t].contains_key(match i {
                                        ExpressionType::Ident { value } => {
                                            value
                                        }
                                        _ => {
                                            red_ln!("Walking error: cannot free anything that is not an identifier");
                                            panic!();
                                        }
                                    }) {
                                        functions[t].remove(match i {
                                            ExpressionType::Ident { value } => {
                                                value
                                            }
                                            _ => unreachable!()
                                        });
                                    }
                                    t += 1;
                                }
                            }
                        }
                        "abort" => {
                            red_ln!(
                                "Process stopped: {:?}",
                                if *&params.len() as i32 == 0 {
                                    String::from("No arguments were provided")
                                } else {
                                    match walk_expression(&params[0], variables, functions) {
                                        VarType::None => String::from("Null"),
                                        VarType::StrExpr { value } => value.to_string(),
                                        VarType::BoolExpr { value } => value.to_string(),
                                        VarType::NumExpr { value } => value.to_string(),
                                    }
                                }
                            );
                            if params.len() > 1 {
                                grey_ln!(
                                    "abort() takes only one argument, the others will be ignored"
                                );
                            }
                            panic!();
                        }
                        "scopes" => {
                            cyan_ln!("[Variables]");
                            let mut t = 0;
                            for i in variables.to_vec() {
                                cyan_ln!("Depth: {}, {:#?}", t, i);
                                t += 1;
                            }
                            t = 0;
                            cyan_ln!("[Functions]");
                            for i in functions.to_vec() {
                                cyan_ln!("Depth: {}, {:#?}", t, i);
                                t += 1;
                            }
                            if params.len() > 0 {
                                grey_ln!(
                                    "No parameters needed, {} parameter{} found",
                                    params.len(),
                                    if params.len() == 1 { "" } else { "s" }
                                );
                            }
                        }
                        _ => {
                            let mut bloc: Vec<Node> = vec![];
                            let mut param: Vec<Parameter> = vec![];
                            for i in functions.clone() {
                                if i.contains_key(identifier) {
                                    bloc = (i.get(identifier).unwrap().block).to_owned();
                                    param = (i.get(identifier).unwrap().params).to_owned();
                                    break;
                                }
                            }
                            walk_block(&bloc, variables, functions, params, &param);
                        }
                    }
                }
            }
        }
        (*variables).remove((*variables).len() - 1);
        (*functions).remove((*functions).len() - 1);
        //Return
        function_result //The main thread returns None internally when it reaches the end
    }
    //Evaluates expressions into values, stored as VarType, for type distinction
    fn walk_expression(
        expr: &ExpressionType,
        mut variables: &mut Vec<HashMap<String, Expression>>,
        mut functions: &mut Vec<HashMap<String, Function>>,
    ) -> VarType {
        match expr {
            ExpressionType::LiteralE { value } => match value {
                Literal::NumberL { value } => VarType::NumExpr {
                    value: value.to_owned() as f32,
                },
                Literal::StringL { value } => VarType::StrExpr {
                    value: value.to_owned(),
                },
                Literal::BooleanL { value } => VarType::BoolExpr {
                    value: value.to_owned(),
                },
            },
            //I did not make a function for analyzing the operators because it didnt seem as safe compared to this approach (tho it sure is a repetitive one)
            ExpressionType::BinaryE { op, lhs, rhs } => {
                let res = match op {
                    Operator::GreaterThan => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                "Walking error: Cannot apply the greater than operator on a Null"
                            );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the greater than operator on a String"
                            );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the greater than operator on a Boolean");
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                "Walking error: Cannot apply the greater than operator on a Null"
                            );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the greater than operator on a String"
                            );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the greater than operator on a Boolean");
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        VarType::BoolExpr { value: l > r }
                    }
                    Operator::LessThan => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                    "Walking error: Cannot apply the less than operator on a Null"
                                );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the less than operator on a String"
                            );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the less than operator on a Boolean"
                            );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                    "Walking error: Cannot apply the less than operator on a Null"
                                );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the less than operator on a String"
                            );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the less than operator on a Boolean"
                            );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        VarType::BoolExpr { value: l < r }
                    }
                    Operator::Ternary => unreachable!(),
                    Operator::Plus => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the plus operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the plus operator on a String"
                                );
                                grey_ln!("Consider using the concat(\"p1\", \"p2\", \"etc\") function");
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the plus operator on a Boolean"
                                );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the plus operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the plus operator on a String"
                                );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the plus operator on a Boolean"
                                );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        VarType::NumExpr { value: l + r }
                    }
                    Operator::Minus => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the minus operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the minus operator on a String"
                                );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the minus operator on a Boolean"
                                );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the minus operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the minus operator on a String"
                                );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the minus operator on a Boolean"
                                );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        VarType::NumExpr { value: l - r }
                    }
                    Operator::Multiplication => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                "Walking error: Cannot apply the multiplication operator on a Null"
                            );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the multiplication operator on a String");
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the multiplication operator on a Boolean");
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                "Walking error: Cannot apply the multiplication operator on a Null"
                            );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the multiplication operator on a String");
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the multiplication operator on a Boolean");
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        VarType::NumExpr { value: l * r }
                    }
                    Operator::Division => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                    "Walking error: Cannot apply the division operator on a Null"
                                );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the division operator on a String"
                                );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the division operator on a Boolean"
                            );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                    "Walking error: Cannot apply the division operator on a Null"
                                );
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!(
                                    "Walking error: Cannot apply the division operator on a String"
                                );
                                panic!()
                            }
                            VarType::BoolExpr { .. } => {
                                red_ln!(
                                "Walking error: Cannot apply the division operator on a Boolean"
                            );
                                panic!()
                            }
                            VarType::NumExpr { value } => value,
                        };
                        VarType::NumExpr { value: l / r }
                    }
                    Operator::And => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the and operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the and operator on a String");
                                panic!()
                            }
                            VarType::BoolExpr { value } => value,
                            VarType::NumExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the and operator on a number");
                                panic!()
                            }
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the and operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the and operator on a String");
                                panic!()
                            }
                            VarType::BoolExpr { value } => value,
                            VarType::NumExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the and operator on a number");
                                panic!()
                            }
                        };
                        VarType::BoolExpr { value: l && r }
                    }
                    Operator::Or => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the or operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the or operator on a String");
                                panic!()
                            }
                            VarType::BoolExpr { value } => value,
                            VarType::NumExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the or operator on a number");
                                panic!()
                            }
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!("Walking error: Cannot apply the or operator on a Null");
                                panic!()
                            }
                            VarType::StrExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the or operator on a String");
                                panic!()
                            }
                            VarType::BoolExpr { value } => value,
                            VarType::NumExpr { .. } => {
                                red_ln!("Walking error: Cannot apply the or operator on a number");
                                panic!()
                            }
                        };
                        VarType::BoolExpr { value: l || r }
                    }
                    Operator::DoubleEqual => {
                        let l = match walk_expression(lhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                "Walking error: Cannot apply the double equal operator on a Null"
                            );
                                panic!()
                            }
                            VarType::StrExpr { value } => value.to_string(),
                            VarType::BoolExpr { value } => value.to_string(),
                            VarType::NumExpr { value } => value.to_string(),
                        };
                        let r = match walk_expression(rhs, &mut variables, &mut functions) {
                            VarType::None => {
                                red_ln!(
                                "Walking error: Cannot apply the double equal operator on a Null"
                            );
                                panic!()
                            }
                            //I understand javascript a bit better now, (true == "true" and 1 == "1")
                            VarType::StrExpr { value } => value.to_string(),
                            VarType::BoolExpr { value } => value.to_string(),
                            VarType::NumExpr { value } => value.to_string(),
                        };
                        VarType::BoolExpr { value: l == r }
                    }
                    Operator::None => {
                        red_ln!("Walking error: the operator is Null");
                        panic!()
                    }
                };
                res
            }
            ExpressionType::UnaryE { op: _op, child } => match *child.to_owned() {
                ExpressionType::LiteralE { value } => match value {
                    Literal::NumberL { value } => VarType::NumExpr {
                        //Since the only unary operator i implement is the negative expression i dont need further checks
                        value: -value.to_owned() as f32,
                    },
                    Literal::StringL { .. } => {
                        red_ln!("Walking error: Cannot apply a unary operator on a string literal");
                        panic!()
                    }
                    Literal::BooleanL { .. } => {
                        red_ln!(
                            "Walking error: Cannot apply a unary operator on a boolean literal"
                        );
                        panic!()
                    }
                },
                _ => unreachable!(),
            },
            ExpressionType::Ident { value } => {
                let mut res = VarType::None;
                for i in variables {
                    if i.contains_key(value) {
                        res = (i.get(value).unwrap().value).to_owned();
                    }
                }
                res
            }
            ExpressionType::FunctionCall { identifier, params } => {
                if identifier.eq("concat") {
                    let mut st = String::new();
                    for i in params {
                        match walk_expression(i, variables, functions) {
                            VarType::None => String::from("Null"),
                            VarType::StrExpr { value } => value.to_string(),
                            VarType::BoolExpr { value } => value.to_string(),
                            VarType::NumExpr { value } => value.to_string(),
                        }.chars().for_each(|c| {
                            st.push(c);
                        });
                    }
                    VarType::StrExpr {
                        value: st.to_string(),
                    }
                } else if identifier.eq("input") {
                    if params.len() == 0 {
                        red_ln!("Walking error: must provide a type argument [\"num\", \"str\"]");
                        panic!()
                    }
                    let mut x = String::from("");
                    stdin().read_line(&mut x).unwrap();
                    match (match &params[0] {
                        ExpressionType::LiteralE { value } => {match value {
                            Literal::NumberL { .. } => {
                                red_ln!("Walking error: must provide a type argument [\"num\", \"str\"]");
                                panic!()}
                            Literal::StringL { value } => value,
                            Literal::BooleanL { .. } => {
                                red_ln!("Walking error: must provide a type argument [\"num\", \"str\"]");
                                panic!()}
                        }}
                        _ => {
                            red_ln!("Walking error: must provide a type argument [\"num\", \"str\"]");
                            panic!()
                        }
                    }).as_str() {
                        "num" => VarType::NumExpr {
                            value: match x.replace("\n", "").parse::<f32>() {
                                Ok(_) => x.replace("\n", "").parse::<f32>().unwrap(),
                                Err(_) => {
                                    red_ln!(
                                        "Walking error: you cannot convert letters to a number"
                                    );
                                    grey_ln!("This includes empty spaces");
                                    panic!()
                                }
                            },
                        },
                        "str" => VarType::StrExpr {
                            value: x.replace("\n", "").to_string(),
                        },
                        _ => {
                            red_ln!("Walking error: must provide a type argument [\"num\", \"str\"]");
                            panic!()
                        }
                    }
                } else if identifier.eq("random_f") || identifier.eq("random") {
                    use rand::Rng;
                    if params.len() > 1 {
                        let mut rng = rand::thread_rng();
                        let randed = rng.gen_range(
                            match walk_expression(&params[0], variables, functions) {
                                VarType::None => {
                                    red_ln!("Walking error: must provide a numeric value");
                                    panic!()
                                }
                                VarType::StrExpr { .. } => {
                                    red_ln!("Walking error: must provide a numeric value");
                                    panic!()
                                }
                                VarType::BoolExpr { .. } => {
                                    red_ln!("Walking error: must provide a numeric value");
                                    panic!()
                                }
                                VarType::NumExpr { value } => value,
                            }
                                ..match walk_expression(&params[1], variables, functions) {
                                    VarType::None => {
                                        red_ln!("Walking error: must provide a numeric value");
                                        panic!()
                                    }
                                    VarType::StrExpr { .. } => {
                                        red_ln!("Walking error: must provide a numeric value");
                                        panic!()
                                    }
                                    VarType::BoolExpr { .. } => {
                                        red_ln!("Walking error: must provide a numeric value");
                                        panic!()
                                    }
                                    VarType::NumExpr { value } => value,
                                },
                        );
                        VarType::NumExpr {
                            value: if !identifier.ends_with("f") {
                                randed.round()
                            } else {
                                randed
                            },
                        }
                    } else {
                        red_ln!("Walking error: an origin and a limit must be specified, other parameters will be ignored");
                        panic!();
                    }
                } else {
                    let mut res = VarType::None;
                    for i in functions.clone() {
                        if i.contains_key(identifier) {
                            res = walk_block(
                                &i.get(identifier).unwrap().block,
                                variables,
                                functions,
                                &params.to_owned(),
                                &i.get(identifier).unwrap().params,
                            );
                        }
                    }
                    //Return
                    res
                }
            }
        }
    }
    fn print(
        params: &Vec<ExpressionType>,
        variables: &mut Vec<HashMap<String, Expression>>,
        functions: &mut Vec<HashMap<String, Function>>,
    ) {
        print!("⨠ ");
        for i in params {
            let result = walk_expression(i, variables, functions);
            print!(
                "{} ",
                match result {
                    VarType::None => {
                        red_ln!("Walking error: unknown variable or function");
                        panic!();
                    }
                    VarType::StrExpr { value } => {
                        value.to_string()
                    }
                    VarType::BoolExpr { value } => {
                        value.to_string()
                    }
                    VarType::NumExpr { value } => {
                        value.to_string()
                    }
                }
            );
        }
    }
    //Begin the process
    walk_block(
        &ast.program,
        &mut variables,
        &mut functions,
        &vec![],
        &vec![],
    );
}
#[derive(Debug, Clone)]
struct Function {
    params: Vec<Parameter>,
    block: Vec<Node>,
}
#[derive(Debug, Clone)]
struct Expression {
    pub value: VarType,
}
#[derive(Debug, Clone)]
enum VarType {
    None,
    StrExpr { value: String },
    BoolExpr { value: bool },
    NumExpr { value: f32 },
}
