use super::super::Parser;
use crate::syntax::ast::{Ast, Expression, Statement};
use crate::syntax::error::ERROR_INDICATOR;
use crate::syntax::lexer::{Token, TokenType, Type};

impl Parser {
    pub fn variable_assignment(&mut self, ret: bool, cursor: Option<usize>) -> Option<Ast> {
        let mut name = self.parse_var_name().unwrap_or(String::new());
        let mut var_type: Option<Type> = self.parse_var_type();

        if cursor.is_some() {
            self.cursor = cursor.unwrap();
        } else {
            self.cursor = self.cursor;
        }

        self.cursor += 2; // Move cursor past '=' and literal_index
        let final_type = match var_type.is_some() {
            true => var_type.unwrap(),
            _ => Type::Void,
        };

        //FIX ME: parse_literal panics when type for tuple is invalid
        //i.e: x: (this)
        //above panics ^^^
        let value = self.parse_literal(self.tokens[self.cursor].clone());
        let variable = Statement::VariableAssignment {
            name,
            var_type: final_type,
            value,
        };

        let mut res = None;

        match ret {
            false => self.nodes.push(Ast::Statement(variable)),
            true => res = Some(Ast::Statement(variable)),
        }

        self.tokens.drain(0..=self.cursor); // Adjusted token removal range

        res
    }

    pub fn parse_var_name(&mut self) -> Option<String> {
        match self.tokens.get(1) {
            Some(token) if token.token_type == TokenType::Identifier => {
                self.cursor += 1;
                Some(token.lexeme.clone())
            }
            Some(token) => {
                let message = format!(
                    "{} \x1b[1mCannot assign variable names to items of type '{:#?}'\x1b[0m",
                    ERROR_INDICATOR, token.token_type
                );
                self.throw_error(token.line_num, message);

                self.tokens.clear();
                None
            }
            None => {
                let message = format!(
                    "{} \x1b[1mUnexpected end of tokens while parsing variable assignment\x1b[0m",
                    ERROR_INDICATOR,
                );
                self.throw_error(0, message);
                None
            }
        }
    }

    pub fn parse_var_type(&mut self) -> Option<Type> {
        let mut var_type: Option<Type> = None;
        let mut var_parse_errors: Vec<(usize, String)> = Vec::new();
        if let Some(token) = self.tokens.get(self.cursor + 1) {
            if token.token_type == TokenType::Colon {
                // Explicit type declaration found
                self.cursor += 2; // Move cursor past ':' and type identifier
                if let Some(next_token) = self.tokens.get(self.cursor) {
                    if next_token.token_type == TokenType::Identifier
                        || next_token.token_type == TokenType::LParen
                    {
                        let type_str = &next_token.lexeme.clone();
                        var_type = match type_str.as_str() {
                            "bool" => Some(Type::Bool),
                            "int" => Some(Type::Int),
                            "short" => Some(Type::Short),
                            "large" => Some(Type::Large),
                            "float" => Some(Type::Float),
                            "string" => Some(Type::String),
                            "array" => {
                                // welcome...
                                Some(Type::Array(Box::new(Type::Void)))
                            }
                            "(" => {
                                let mut types = Vec::new();
                                while let Some(tup_type) = self.tokens.get(self.cursor + 1) {
                                    println!("{}", tup_type.lexeme.clone());
                                    if tup_type.token_type != TokenType::Identifier {
                                        let message = format!(
                                            "{} Unknown type '{:#?}' found in tuple '{}'",
                                            ERROR_INDICATOR,
                                            tup_type.token_type,
                                            self.tokens[0].lexeme.clone()
                                        );
                                        var_parse_errors.push((tup_type.line_num, message));
                                    }
                                    let tup_type_str = &tup_type.lexeme.clone();
                                    let tup_repr = match tup_type_str.as_str() {
                                        "bool" => Type::Bool,
                                        "int" => Type::Int,
                                        "short" => Type::Short,
                                        "large" => Type::Large,
                                        "float" => Type::Float,
                                        "string" => Type::String,
                                        "array" => Type::Array(Box::new(Type::Bool)),
                                        "(" => {
                                            let message = format!("{} \x1b[1mNested Tuples are not currently supported\x1b[0m", ERROR_INDICATOR);
                                            var_parse_errors.push((tup_type.line_num, message));
                                            break;
                                        }
                                        "void" => Type::Void,
                                        _ => {
                                            let message = format!(
                                                "{} Unknown type '{:#?}' found in tuple '{}'",
                                                ERROR_INDICATOR,
                                                tup_type.token_type,
                                                self.tokens[0].lexeme.clone()
                                            );
                                            var_parse_errors.push((tup_type.line_num, message));
                                            break;
                                        }
                                    };
                                    types.push(tup_repr);
                                    self.cursor += 2;

                                    if let Some(next_comma) = self.tokens.get(self.cursor) {
                                        match next_comma.token_type {
                                            TokenType::RParen => break,
                                            TokenType::Comma => self.cursor += 0, // idk how this works bruh
                                            _ => continue,
                                        }
                                    }
                                }
                                Some(Type::Tuple(types))
                            }
                            "void" => Some(Type::Void),
                            _ => None,
                        };
                        if var_type.is_none() {
                            let message =
                                format!("{} \x1b[1mUnknown type '{}'", ERROR_INDICATOR, type_str);

                            var_parse_errors.push((token.line_num, message));
                        };
                    } else {
                        let message = format!(
                            "{} \x1b[1mUnknown type '{}'",
                            ERROR_INDICATOR, self.tokens[self.cursor].lexeme
                        );
                        var_parse_errors.push((self.tokens[self.cursor].line_num, message));
                    }
                }
            } else if token.token_type == TokenType::Eq {
                var_type = None;
            }
        } else {
            let message = format!(
                "{} \x1b[1mUnexpected end of tokens while parsing variable assignment\x1b[0m",
                ERROR_INDICATOR,
            );
            var_parse_errors.push((0, message))
        }
        if var_parse_errors.len() > 0 {
            for i in 0..var_parse_errors.len() {
                let line = var_parse_errors[i].0;
                let message = format!("{}", &var_parse_errors[i].1);
                self.throw_error(var_parse_errors[i].0, message);
                self.tokens.clear()
            }
        }
        var_type
    }
}
