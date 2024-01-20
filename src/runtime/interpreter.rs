use std::io;

use super::environment::Environment;
use crate::syntax::ast::{Ast, Expression, Statement};

pub fn evaluate(nodes: Vec<Ast>) {
    let mut env = Environment::init();
    for node in nodes {
        match node {
            Ast::Expression(expr) => match expr {
                Expression::CallExpr { name, params } => {
                    println!("starting call expr eval");
                    println!("{:#?}", name);
                    let mut is_lib = false;

                    for lib in env.lib_functions.iter() {
                        if name == lib.name {
                            is_lib = true
                        } else {
                            continue;
                        }
                    }

                    if is_lib {
                        match name.as_str() {
                            "print" => match &params[0] {
                                Expression::Identifier(ident) => {
                                    if let Some(expr) = env.variables.get(ident) {
                                        println!("The Identifier is `{}`", &ident);
                                        match expr {
                                            Expression::StringLiteral(str) => {
                                                println!("{}", str)
                                            }
                                            _ => todo!(),
                                        }
                                    } else {
                                        println!("{:#?}", &params[0])
                                    }
                                }
                                Expression::StringLiteral(ident) => {
                                    println!("{}", ident)
                                }
                                _ => todo!(),
                            },
                            "input" => {
                                let mut buffer = String::new();
                                io::stdin()
                                    .read_line(&mut buffer)
                                    .expect("Failed to read line");
                                let buffer = buffer.trim().to_string();

                                match &params[0] {
                                    Expression::Identifier(ident) => env.declare_variable(
                                        ident.to_string(),
                                        Expression::StringLiteral(buffer),
                                        false,
                                    ),
                                    _ => unimplemented!(),
                                };
                                println!("{:#?}", env);
                                continue;
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                Expression::Null => {}
                _ => unimplemented!(), // sticking out your gyat
            },
            Ast::Statement(stmt) => match stmt {
                Statement::VariableAssignment {
                    constant,
                    name,
                    var_type: _, // gonna remove this eventually
                    value,
                } => {
                    env.declare_variable(name, value, constant);
                }
                _ => todo!(),
            },
        }
    }
    println!("{:#?}", env)
}
