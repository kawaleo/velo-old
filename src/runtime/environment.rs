use std::collections::{HashMap, HashSet};

use crate::error::{ErrorType::RuntimeError, VeloError, ERROR_INDICATOR};
use crate::syntax::ast::Expression;

#[derive(Debug)]
pub struct Environment {
    errors: Vec<VeloError>,
    parent: Option<Box<Environment>>,
    variables: HashMap<String, Expression>,
    constants: HashSet<Expression>,
}

impl Environment {
    pub fn generate_env() -> Self {
        todo!()
    }

    fn declare_variable(&mut self, name: String, value: Expression, constant: bool) -> Expression {
        if constant {
            todo!()
        }

        Expression::Null
    }

    pub fn throw_error(&mut self, message: String) {
        let message = format!("{} \x1b[1m{}\x1b[0m", ERROR_INDICATOR, message);
        self.errors
            .push(VeloError::error(0, &message, RuntimeError));
    }
}
