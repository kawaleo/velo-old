use super::super::Parser;
use crate::syntax::ast::{Ast, Expression, FunctionBody, Literal, Statement};
use crate::syntax::error::ERROR_INDICATOR;
use crate::syntax::lexer::{Token, TokenType, Type};

impl Parser {
    pub fn function_declaration(&mut self) {
        let mut name = self.parse_function_name();
        let mut params = self.parse_function_params(&name);
        let mut ret_type = self.parse_function_ret_type(&name);

        let mut stmts = Vec::new();
        let mut exprs = Vec::new();
        let mut block = None;
        // Return type

        // make variable_assignment not drain tokens when parsing for function body
        // MOST CLEAN CODE OF 2024 (official)
        if let Some(token) = self.tokens.get(self.cursor - 1) {
            println!("{}", token.lexeme.clone());
            if token.token_type == TokenType::LBrace || token.token_type == TokenType::Identifier {
                let mut brace_count = 1;
                let mut body_cursor = self.cursor + 1;

                match token.token_type {
                    TokenType::LBrace => body_cursor = self.cursor,
                    _ => body_cursor = self.cursor + 1,
                }

                while let Some(body_token) = self.tokens.get(body_cursor) {
                    println!("body token {}", body_token.lexeme);
                    match body_token.token_type {
                        TokenType::LBrace => brace_count += 1,
                        TokenType::RBrace => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                self.cursor = body_cursor + 1;
                                break;
                            }
                        }
                        TokenType::Identifier => match self.tokens[body_cursor + 1].token_type {
                            TokenType::ColonEq => {
                                body_cursor += 1;
                                let var =
                                    self.variable_assignment(true, Some(body_cursor), false, true);
                                stmts.push(var.unwrap())
                            }
                            _ => unimplemented!(),
                        },

                        _ => {}
                    }

                    body_cursor += 1;
                }
            } else {
                let message = format!(
                    "{} \x1b[1mExpected '{{' to start function body, found '{}'\x1b[0m",
                    ERROR_INDICATOR,
                    token.lexeme.clone()
                );
                self.throw_error(token.line_num, message);

                self.tokens.clear();
                return;
            }
        } else {
            let message = format!(
                "{} \x1b[1mUnexpected end of tokens while parsing function body\x1b[0m",
                ERROR_INDICATOR,
            );
            self.throw_error(0, message);

            return;
        }

        let body = FunctionBody::new(stmts, exprs, block);

        self.tokens.drain(0..self.cursor);
        self.cursor = 0;
        // Avengers! Assemble (please help)
        let function_assignment = Statement::Function {
            name,
            params,
            ret_type,
            body,
        };
        self.nodes.push(Ast::Statement(function_assignment));
    }

    fn parse_function_name(&mut self) -> String {
        let mut name = String::new();

        match self.tokens.get(1) {
            Some(next_token) if next_token.token_type == TokenType::Identifier => {
                name = next_token.lexeme.clone();
                self.cursor += 1;
            }
            Some(next_token) => {
                let message = format!(
                    "{} \x1b[1mCannot declare function with name of type {}",
                    ERROR_INDICATOR,
                    TokenType::to_string(next_token.token_type)
                );
                self.throw_error(next_token.line_num, message);
            }
            None => {
                let message = format!(
                    "{} \x1b[1mUnexpected end of input while parsing function declaration\x1b[0m",
                    ERROR_INDICATOR,
                );
                self.throw_error(self.tokens[0].line_num, message);
            }
        }

        name
    }

    fn parse_function_params(&mut self, name: &String) -> Vec<(String, Type)> {
        let mut params = Vec::new();

        if let Some(token) = self.tokens.get(self.cursor + 1) {
            if token.token_type == TokenType::LParen {
                let mut param_cursor = self.cursor + 2;
                while let Some(param_token) = self.tokens.get(param_cursor) {
                    if param_token.token_type == TokenType::RParen {
                        self.cursor = param_cursor + 1;
                        break;
                    } else if param_token.token_type == TokenType::Identifier {
                        // Parsing parameter name
                        let param_name = param_token.lexeme.clone();

                        if let Some(next_token) = self.tokens.get(param_cursor + 1) {
                            // Checking for parameter type declaration
                            if next_token.token_type == TokenType::Identifier {
                                if let Some(type_token) = self.tokens.get(param_cursor + 1) {
                                    // Parsing parameter type
                                    let param_type = Type::from_string(type_token.lexeme.clone());
                                    params.push((param_name, param_type));

                                    param_cursor += 2; // Move cursor past parameter type
                                    if let Some(next_next_token) = self.tokens.get(param_cursor) {
                                        match next_next_token.token_type {
                                            TokenType::Comma => param_cursor += 1,
                                            TokenType::RParen => {
                                                self.cursor = param_cursor + 1;
                                                break;
                                            }
                                            _ => {
                                                // Handle unexpected token
                                                let message = format!(
                                                "{} \x1b[1mUnexpected token '{:#?}' while parsing parameters for function '{}'\x1b[0m",
                                                ERROR_INDICATOR, next_next_token.token_type, name
                                            );
                                                self.throw_error(next_next_token.line_num, message);
                                                self.tokens.drain(0..param_cursor);
                                                break;
                                            }
                                        }
                                    }
                                } else {
                                    // Handle missing parameter type
                                    let message = format!(
                                        "{} \x1b[1mExpected parameter type after '{}'\x1b[0m",
                                        ERROR_INDICATOR, param_name
                                    );
                                    self.throw_error(next_token.line_num, message);
                                    self.tokens.clear();
                                    self.cursor += 1;
                                    break;
                                }
                            } else {
                                let message = format!(
                                    "{} \x1b[1mExpected type to follow parameter, but found '{}' for function '{}'",
                                    ERROR_INDICATOR,
                                    next_token.lexeme.clone(),
                                    name,
                                );
                                self.throw_error(next_token.line_num, message);
                                self.cursor += 1;
                                break;
                            }
                        }
                    } else {
                        // Handle unexpected token for parameter name
                        let message = format!(
                        "{} \x1b[1mUnexpected token '{:#?}' while parsing parameters for function '{}'\x1b[0m",
                        ERROR_INDICATOR, TokenType::to_string(param_token.token_type), name
                    );
                        self.throw_error(param_token.line_num, message);
                        self.tokens.clear();
                        break;
                    }
                }
            } else {
                // Handle missing '(' after function name
                let message = format!(
                    "{} \x1b[1mExpected '(' after function name, found '{:#?}' for function '{}'\x1b[0m",
                    ERROR_INDICATOR, 
                    TokenType::to_string(token.token_type),
                    name
                );
                self.throw_error(token.line_num, message);
                self.tokens.clear();
            }
        } else {
            // Handle unexpected end of tokens while parsing function parameters
            let message = format!(
                "{} \x1b[1mUnexpected end of tokens while parsing parameters for function '{}'\x1b[0m",
                ERROR_INDICATOR, name
            );
            self.throw_error(0, message);
        }
        params
    }

    fn parse_function_ret_type(&mut self, name: &String) -> Type {
        let mut ret_type = Type::Void;

        if let Some(token) = self.tokens.get(self.cursor) {
            if token.token_type == TokenType::Gt {
                if let Some(next_token) = self.tokens.get(self.cursor + 1) {
                    ret_type = Type::from_string(next_token.lexeme.clone());
                    self.cursor += 2;
                } else {
                    let message = format!(
                        "{} \x1b[1mExpected return type after '>' for function '{}'\x1b[0m",
                        ERROR_INDICATOR, name
                    );
                    self.throw_error(token.line_num, message);
                    self.tokens.clear();
                }
            } else {
                match token.token_type {
                    TokenType::LBrace => {
                        self.cursor += 1;
                    }
                    _ => {
                        let message = format!("{} \x1b[1mExpected either '>' or '{}' when parsing function '{}', but found {}", ERROR_INDICATOR, "{",  name, token.lexeme.clone());
                        self.throw_error(token.line_num, message);
                        self.tokens.clear();
                    }
                }
            }
        } else {
            let message = format!(
                "{} \x1b[1mUnexpected end of tokens while parsing return type for function '{}'\x1b[0m",
                ERROR_INDICATOR, name
            );
            self.throw_error(0, message);
        }
        ret_type
    }
}
