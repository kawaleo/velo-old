// todo??
use super::environment::Environment;
use crate::syntax::ast::{Ast, Expression, Statement};

pub fn evaluate(nodes: Vec<Ast>) {
    let mut env = Environment::init();
    for node in nodes {
        match node {
            Ast::Expression(expr) => match expr {
                Expression::CallExpr { name, params } => {
                    println!("evaluating call expr");
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
                            "print" => println!("{:#?}", params[0]),
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
                    var_type,
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
