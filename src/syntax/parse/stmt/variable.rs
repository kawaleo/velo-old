use super::super::Parser;
use crate::syntax::ast::{Ast, Expression, Statement};
use crate::syntax::error::ERROR_INDICATOR;
use crate::syntax::lexer::{Token, TokenType, Type};

impl Parser {
    pub fn variable_assignment(
        &mut self,
        ret: bool,
        cursor: Option<usize>,
        mk_const: bool,
    ) -> Option<Ast> {
        let mut name = self.parse_var_name(mk_const);

        let mut infer_type = true;

        if cursor.is_some() {
            self.cursor = cursor.unwrap();
        } else {
            self.cursor = self.cursor;
        }

        self.cursor += 2; // Move cursor past '=' and literal_index

        let mut final_type = Type::Void;

        //FIX ME: parse_literal panics when type for tuple is invalid
        //i.e: x: (this)
        //above panics ^^^
        let value = self.parse_literal(self.tokens[self.cursor].clone(), &Type::Void, infer_type);

        match value.1.is_some() {
            true => final_type = value.1.unwrap(),
            _ => {}
        }
        let message = format!(
            "{} \x1b[1mExpected semicolon following variable '{}'\x1b[0m",
            ERROR_INDICATOR, name
        );

        let variable = Statement::VariableAssignment {
            constant: mk_const,
            name,
            var_type: final_type,
            value: value.0,
        };

        let mut res = None;

        match ret {
            false => self.nodes.push(Ast::Statement(variable)),
            true => res = Some(Ast::Statement(variable)),
        }
        self.tokens.drain(0..=self.cursor); // Adjusted token removal range
        self.cursor = 0;

        res
    }

    pub fn parse_var_name(&mut self, is_const: bool) -> String {
        if !is_const {
            let name = self.tokens[0].lexeme.clone();
            name
        } else {
            let name = self.tokens[1].lexeme.clone(); // todo handle case where no name
            self.cursor += 1;
            name
        }
    }
}
