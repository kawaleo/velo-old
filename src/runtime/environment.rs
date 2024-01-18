use std::collections::HashMap;

use crate::error::{ErrorType::RuntimeError, VeloError, ERROR_INDICATOR};
use crate::syntax::ast::{Expression, Statement};

#[derive(Debug, Clone)]
pub struct Environment {
    pub errors: Vec<VeloError>,
    pub parent: Option<Box<Environment>>,
    pub variables: HashMap<String, Expression>,
    pub constants: Vec<Expression>,
    pub functions: Vec<Statement>,
    pub lib_functions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct LibFunction {
    pub name: String,
    pub param_len: Option<usize>,
}

impl Environment {
    pub fn init() -> Self {
        let mut lib_functions: Vec<String> = vec!["print"].iter().map(|f| f.to_string()).collect();

        Environment {
            errors: Vec::new(),
            parent: None,
            variables: HashMap::new(),
            constants: Vec::new(),
            functions: Vec::new(),
            lib_functions,
        }
    }

    fn declare_variable(&mut self, name: String, value: Expression, constant: bool) -> Expression {
        if constant {
            self.constants.push(value)
            // yk i would make constants a hash set
            // but that doesnt work with f32 for some weird reason
            // i love rust :)
        } else {
            if self.variables.contains_key(&name) {
                let message = format!("Variable with name '{}' already exists, did you mean to use `:=` instead of `=`?", &name);
                self.throw_error(message)
            } else {
                self.variables.insert(name, value);
            }
        }

        Expression::Null
    }

    fn throw_error(&mut self, message: String) {
        let message = format!("{} \x1b[1m{}\x1b[0m", ERROR_INDICATOR, message);
        self.errors
            .push(VeloError::error(0, &message, RuntimeError));
    }
}