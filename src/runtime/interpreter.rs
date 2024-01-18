// todo??
use super::environment::Environment;
use crate::syntax::ast::{Ast, Expression, Statement};

pub fn evaluate(nodes: Vec<Ast>, env: &mut Environment) {
    for node in nodes {
        match node {
            Ast::Expression(expr) => match expr {
                Expression::CallExpr { name, params } => {
                    println!("evaluating call expr");
                    let is_lib = env.lib_functions.contains(&name);

                    if is_lib {
                        match name.as_str() {
                            "print" => println!("{}", params[0]),
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
                } => todo!(),
                _ => todo!(),
            },
            _ => unimplemented!(),
        }
    }
}
