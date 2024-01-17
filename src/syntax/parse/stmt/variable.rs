use super::super::Parser;
use crate::syntax::ast::{Ast, Expression, Literal, Statement};
use crate::syntax::error::ERROR_INDICATOR;
use crate::syntax::lexer::{Token, TokenType, Type};

impl Parser {
    pub fn variable_assignment(
        &mut self,
        ret: bool,
        cursor: Option<usize>,
        mk_const: bool,
        in_fn: bool,
    ) -> Option<Statement> {
        println!("{:#?}", self.tokens.clone());
        let mut name = self.parse_var_name(mk_const, in_fn);

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
        let mut value = (Expression::Literal(Literal::Null), None);
        let tok = match in_fn {
            true => self.tokens[self.cursor - 1].clone(),
            _ => self.tokens[self.cursor].clone(),
        };

        value = self.parse_literal(tok, &Type::Void, infer_type);

        match value.1.is_some() {
            true => final_type = value.1.unwrap(),
            _ => {}
        }
        let message = format!(
            "{} \x1b[1mExpected semicolon following variable '{}'\x1b[0m",
            ERROR_INDICATOR, name
        );

        self.expect(TokenType::Semicolon, self.cursor, message);
        self.cursor += 1;

        let variable = Statement::VariableAssignment {
            constant: mk_const,
            name,
            var_type: final_type,
            value: value.0,
        };

        let mut res = None;

        match ret {
            false => self.nodes.push(Ast::Statement(variable)),
            true => res = Some(variable),
        }
        if !in_fn {
            self.tokens.drain(0..=self.cursor); // Adjusted token removal range
            self.cursor = 0;
        }

        res
    }

    pub fn parse_var_name(&mut self, is_const: bool, in_fn: bool) -> String {
        if !is_const {
            let name = self.tokens[self.cursor].lexeme.clone();
            name
        } else {
            let name = self.tokens[1].lexeme.clone(); // todo handle case where no name
            self.cursor += 1;
            name
        }
    }
}
