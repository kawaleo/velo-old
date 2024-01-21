use super::environment::Environment;
use super::eval::expr::*;
use crate::syntax::ast::{Ast, Expression, Statement};

use std::io;

// TODO: Create a separate call expr function that just returns an expression
// I'll use it when parsing variables probably
pub fn evaluate(nodes: Vec<Ast>) {
    let mut env = Environment::init();
    for node in nodes {
        match node {
            Ast::Expression(expr) => match expr {
                Expression::CallExpr {
                    ref name,
                    ref params,
                } => eval_call_expr(&expr, &mut env, None),
                Expression::Null => {}
                _ => unimplemented!(), // sticking out your gyat
            },
            Ast::Statement(stmt) => match stmt {
                Statement::VariableAssignment {
                    constant,
                    ref name,
                    ref value,
                } => match value {
                    Expression::CallExpr { name, params } => {
                        eval_call_expr(&value, &mut env, Some(&stmt))
                    }
                    _ => {
                        env.declare_variable(name.to_string(), value.clone(), constant);
                    }
                },
                _ => todo!(),
            },
        }
    }
    println!("{:#?}", env)
}
