// todo??
use super::environment::Environment;
use crate::syntax::ast::{Ast, Expression, Statement};

pub fn evaluate(node: Ast, env: Environment) {
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
        _ => unimplemented!(),
    }
}
