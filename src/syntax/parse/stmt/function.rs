use super::super::Parser;
use crate::syntax::ast::{Ast, Expression, Literal, Statement};
use crate::syntax::error::ERROR_INDICATOR;
use crate::syntax::lexer::{Token, TokenType, Type};

impl Parser {
    pub fn function_declaration(&mut self) {
        let mut name = String::new();
        let mut params = Vec::new();
        let mut body = Vec::new();
        let mut ret_type = Type::Void;

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

                return;
            }
        }
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
                            if next_token.token_type == TokenType::Colon {
                                if let Some(type_token) = self.tokens.get(param_cursor + 2) {
                                    // Parsing parameter type
                                    let param_type = Type::from_string(type_token.lexeme.clone());
                                    params.push((param_name, param_type));

                                    param_cursor += 3; // Move cursor past parameter type
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
                                        "{} \x1b[1mExpected parameter type after ':'\x1b[0m",
                                        ERROR_INDICATOR,
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
                return;
            }
        } else {
            // Handle unexpected end of tokens while parsing function parameters
            let message = format!(
                "{} \x1b[1mUnexpected end of tokens while parsing parameters for function '{}'\x1b[0m",
                ERROR_INDICATOR, name
            );
            self.throw_error(0, message);
            return;
        }

        // Return type
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
                    return;
                }
            } else {
                match token.token_type {
                    TokenType::LBrace => {}
                    _ => {
                        let message = format!("{} \x1b[1mExpected either '>' or '{}' when parsing function '{}', but found {}", ERROR_INDICATOR, "{",  name, token.lexeme.clone());
                        self.throw_error(token.line_num, message);
                        self.tokens.clear();
                        return;
                    }
                }
            }
        } else {
            let message = format!(
                "{} \x1b[1mUnexpected end of tokens while parsing return type for function '{}'\x1b[0m",
                ERROR_INDICATOR, name
            );
            self.throw_error(0, message);
            return;
        }

        // Function body
        if let Some(token) = self.tokens.get(self.cursor) {
            if token.token_type == TokenType::LBrace {
                let mut brace_count = 1;
                let mut body_cursor = self.cursor + 1;

                while let Some(body_token) = self.tokens.get(body_cursor) {
                    match body_token.token_type {
                        TokenType::LBrace => brace_count += 1,
                        TokenType::RBrace => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                self.cursor = body_cursor + 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                    body.push(Ast::Expression(Expression::Literal(Literal::Null)));
                    body_cursor += 1;
                }
            } else {
                let message = format!(
                    "{} \x1b[1mExpected '{{' to start function body, found '{:#?}'\x1b[0m",
                    ERROR_INDICATOR, token.token_type
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

        self.tokens.drain(0..self.cursor);
        // Avengers! Assemble (please help)
        let function_assignment = Statement::Function {
            name,
            params,
            ret_type,
            body,
        };
        self.nodes.push(Ast::Statement(function_assignment));
    }
}
