use super::super::Parser;
use crate::syntax::ast::{Ast, Expression};
use crate::syntax::lexer::TokenType;

impl Parser {
    pub fn call_expr(&mut self) -> Expression {
        println!("Parsing a standard call expression");
        let name = self.tokens[self.cursor].lexeme.clone();
        self.cursor += 2;

        let mut params = Vec::new();

        while let Some(param_token) = self.tokens.get(self.cursor) {
            match param_token.token_type {
                TokenType::String => {
                    params.push(Expression::StringLiteral(param_token.lexeme.clone()));
                    self.cursor += 1;

                    if let Some(next_token) = self.tokens.get(self.cursor) {
                        println!("{}", next_token.lexeme.clone());
                        match next_token.token_type {
                            TokenType::Comma => self.cursor += 1,
                            TokenType::RParen => {
                                self.cursor += 1;
                                break;
                            }
                            _ => unimplemented!(),
                        }
                    } else {
                        std::process::exit(1)
                    }
                }
                TokenType::Identifier => {
                    let literal = self.parse_literal(param_token.clone(), Some(self.cursor));
                    self.cursor += 1;

                    params.push(literal);
                    if let Some(after_token) = self.tokens.get(self.cursor) {
                        println!("{}", after_token.lexeme.clone());
                        match after_token.token_type {
                            TokenType::Comma => self.cursor += 1,
                            TokenType::RParen => {
                                self.cursor += 1;
                                break;
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                TokenType::RParen => break,

                _ => {
                    println!("`{}` is unimplemented!", param_token.lexeme.clone());
                    unimplemented!()
                }
            }
        }
        let temp = params.clone();
        let nm = format!("{}", &name);
        let call_expr = Expression::CallExpr { name, params };

        self.tokens.drain(0..self.cursor); // so uhh... forgot to add this line...
                                           // took 2 hours to figure out why it wasnt working
                                           // having fun :)

        self.cursor = 0; // once again, forgot to add this line
                         // took me around 30 minutes before i walked away
                         // literally figured out the error while rock climbing... lol
        self.nodes.push(Ast::Expression(call_expr));
        Expression::CallExpr {
            name: nm,
            params: temp,
        }
    }

    pub fn call_expr_as_var(&mut self) -> Expression {
        let name = self.tokens[self.cursor].lexeme.clone();
        self.cursor += 3;
        println!("{:#?}", self.tokens[self.cursor].lexeme.clone());
        let mut params = Vec::new();

        while let Some(param_token) = self.tokens.get(self.cursor) {
            match param_token.token_type {
                TokenType::String => {
                    params.push(Expression::StringLiteral(param_token.lexeme.clone()));
                    self.cursor += 1;

                    if let Some(next_token) = self.tokens.get(self.cursor) {
                        println!("{}", next_token.lexeme.clone());
                        match next_token.token_type {
                            TokenType::Comma => self.cursor += 1,
                            TokenType::RParen => {
                                self.cursor += 1;
                                break;
                            }
                            _ => unimplemented!(),
                        }
                    } else {
                        std::process::exit(1)
                    }
                }
                TokenType::Identifier => {
                    let literal = self.parse_literal(param_token.clone(), Some(self.cursor));
                    self.cursor += 1;

                    params.push(literal);
                    if let Some(after_token) = self.tokens.get(self.cursor) {
                        println!("{}", after_token.lexeme.clone());
                        match after_token.token_type {
                            TokenType::Comma => self.cursor += 1,
                            TokenType::RParen => {
                                self.cursor += 1;
                                break;
                            }
                            _ => unimplemented!(),
                        }
                    }
                }
                TokenType::RParen => break,
                _ => {
                    println!("`{}` is unimplemented!", param_token.lexeme.clone());
                    unimplemented!()
                }
            }
        }
        let call_expr = Expression::CallExpr { name, params };

        self.tokens.drain(0..=self.cursor); // so uhh... forgot to add this line...
                                            // took 2 hours to figure out why it wasnt working
                                            // having fun :)

        self.cursor = 0;

        call_expr
    }
}