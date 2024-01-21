use super::super::environment::Environment;
use crate::syntax::ast::{Expression, Statement};

use std::io;

pub fn eval_call_expr(call_expr: &Expression, env: &mut Environment, var: Option<&Statement>) {
    let mut nm = String::new();
    let mut pm: Vec<Expression> = Vec::new();
    match call_expr {
        Expression::CallExpr { name, params } => {
            nm = name.to_string();
            pm = params.to_vec();
        }
        _ => unreachable!(),
    }
    let mut is_lib = false;

    let name = nm.clone();
    let params = pm.clone();

    for lib in env.lib_functions.iter() {
        if name == lib.name {
            is_lib = true
        } else {
            continue;
        }
    }

    if is_lib {
        match name.as_str() {
            "print" | "println" => {
                let mut line = false;
                if name.as_str() == "println" {
                    line = true
                }
                match &params[0] {
                    Expression::Identifier(ident) => {
                        if let Some(expr) = env.variables.get(ident) {
                            match expr {
                                Expression::StringLiteral(str) => {
                                    if line {
                                        println!("{}", str)
                                    } else {
                                        print!("{}", str)
                                    }
                                }
                                _ => {
                                    println!("{:#?}", expr);
                                    todo!()
                                }
                            }
                        } else {
                            std::process::exit(1)
                        }
                    }
                    Expression::StringLiteral(ident) => {
                        if line {
                            println!("{}", ident)
                        } else {
                            print!("{}", ident)
                        }
                    }
                    _ => todo!(),
                }
            }
            "input" => {
                if var.is_none() {
                    println!("`input` requires a variable to store to");
                }
                let mut buffer = String::new();
                io::stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to read line");
                let buffer = buffer.trim().to_string();

                let var = var.unwrap();
                match var {
                    Statement::VariableAssignment {
                        constant,
                        name,
                        value: _,
                    } => env.declare_variable(
                        format!("{}", &name),
                        Expression::StringLiteral(buffer),
                        *constant,
                    ),
                    _ => {
                        println!("Idk how this error happens, but if someone gets it, explain what you did please");
                        std::process::exit(1)
                    }
                };
            }
            _ => unimplemented!(),
        }
    }
}
