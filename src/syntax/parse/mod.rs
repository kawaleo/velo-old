#![allow(unused)]
mod stmt;

use super::ast::Expression;
use super::ast::*;
use super::error::{ErrorType::ParseError, VeloError, ERROR_INDICATOR};
use super::lexer::{KeywordMap, Token, TokenType, Type, KEYWORDS};
use stmt::variable;

use std::process;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub cursor: usize,
    pub nodes: Vec<Ast>,
    pub errors: Vec<VeloError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            cursor: 0,
            nodes: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Ast>, VeloError> {
        while !self.tokens.is_empty() {
            match self.tokens[0].token_type {
                TokenType::Let => {
                    self.variable_assignment(false, None);
                }
                TokenType::Func => self.function_declaration(),
                TokenType::EOF => {
                    self.nodes
                        .push(Ast::Expression(Expression::Literal(Literal::Null)));
                    self.tokens.remove(0);
                }
                _ => {
                    println!(
                        "incomplete\n{:#?}\n{:#?}",
                        self.tokens[0].token_type,
                        self.tokens[0].lexeme.clone()
                    );
                    process::exit(1)
                }
            };
        }

        let mut ast_nodes = Vec::new();

        for node in &self.nodes {
            ast_nodes.push(node.clone())
        }
        if self.errors.len() > 0 {
            for error in self.errors.iter() {
                println!("{}", error.message);
                println!("  [filename goes here]:{}\n\n", error.line);
                println!("TODO: Potential Fixes");

                match error.error_type {
                    ParseError => println!("This error is found to be of type `ParseError`"),
                    _ => unreachable!(),
                }
            }
            process::exit(1);
        }

        Ok(ast_nodes)
    }

    fn parse_literal(&mut self, token: Token) -> Expression {
        match token.token_type {
            TokenType::String => Expression::Literal(Literal::StringLiteral(token.lexeme.clone())),
            TokenType::NumericLiteral => {
                let has_operator = self.tokens.get(self.cursor + 1).map_or(false, |t| {
                    matches!(
                        t.token_type,
                        TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div
                    )
                });

                let mut value = token.lexeme.parse::<i32>().map_err(|_| {
                    let message = format!(
                        "{} \x1b[1mFailed to parse '{}' as a numeric literal\x1b[0m",
                        ERROR_INDICATOR, token.lexeme,
                    );

                    self.throw_error(self.tokens[0].line_num, message);
                });

                if has_operator {
                    let mut to_eval: Vec<String> = Vec::new();

                    #[allow(unused)]
                    let mut includes_identifiers = false;
                    let mut keyword_error = false;
                    let mut keyword_fault = TokenType::Null;
                    let mut current_index = self.cursor + 1;

                    to_eval.push(self.tokens[current_index - 1].lexeme.clone());

                    while let Some(next_token) = self.tokens.get(current_index) {
                        if [
                            TokenType::NumericLiteral,
                            TokenType::Identifier,
                            TokenType::Add,
                            TokenType::Sub,
                            TokenType::Mul,
                            TokenType::Div,
                        ]
                        .contains(&next_token.token_type)
                            || KeywordMap::get(&KEYWORDS, &next_token.lexeme).is_some()
                        {
                            if KEYWORDS.get(&next_token.lexeme).is_some() {
                                keyword_error = true;
                                keyword_fault = next_token.token_type;
                                println!("keyword: {:#?}", KEYWORDS.get(&next_token.lexeme))
                            }
                            if next_token.token_type == TokenType::Identifier {
                                to_eval.push(next_token.lexeme.clone());
                                includes_identifiers = true;
                                current_index += 1;
                            } else {
                                to_eval.push(next_token.lexeme.clone());
                                current_index += 1;
                            }
                        } else {
                            break;
                        }
                    }

                    let keyword_error_msg = format!(
                        "{} \x1b[1mExpected ';' after expression, found keyword '{}'\x1b[0m",
                        ERROR_INDICATOR,
                        TokenType::to_string(keyword_fault),
                    );

                    self.cursor = current_index - 1;

                    match keyword_error {
                        false => value = Ok(Self::evaluate_expression(&to_eval)),
                        _ => {
                            value = Ok(0);
                            self.throw_error(self.tokens[1].line_num, keyword_error_msg);
                        }
                    }

                    Expression::Literal(Literal::NumericLiteral(value.unwrap_or(0)))
                } else {
                    // We can't parse the expression on the spot if an identifier is included
                    // This means we have to wait until the environment can access variables to build a BinaryExpression
                    Expression::Literal(Literal::NumericLiteral(value.unwrap_or(0)))
                }

                // For now, just parse the current token as i32
            }
            _ => {
                let message = format!(
                    "{} \x1b[1mCannot assign items of type {:#?} to variables\x1b[0m",
                    ERROR_INDICATOR, token.token_type
                );
                self.throw_error(self.tokens[0].line_num, message);
                self.cursor = 0;

                Expression::Literal(Literal::Null)
            }
        }
    }

    fn evaluate_expression(expr: &[String]) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        let mut ops: Vec<&str> = Vec::new();

        let mut i = 0;
        while i < expr.len() {
            match expr[i].as_str() {
                "+" | "-" | "*" | "/" => {
                    ops.push(&expr[i]);
                }
                _ => {
                    let num = expr[i].parse::<i32>();
                    if let Ok(value) = num {
                        nums.push(value);
                    }
                }
            }
            i += 1;
        }

        // First, perform multiplication and division
        let mut j = 0;
        while j < ops.len() {
            if ops[j] == "*" {
                let res = nums[j] * nums[j + 1];
                nums[j] = res;
                nums.remove(j + 1);
                ops.remove(j);
            } else if ops[j] == "/" {
                let res = nums[j] / nums[j + 1];
                nums[j] = res;
                nums.remove(j + 1);
                ops.remove(j);
            } else {
                j += 1;
            }
        }

        // Then, perform addition and subtraction
        let mut result = nums[0];
        for k in 0..ops.len() {
            if ops[k] == "+" {
                result += nums[k + 1];
            } else if ops[k] == "-" {
                result -= nums[k + 1];
            }
        }

        result
    }

    fn parse_expression(_tokens: &Vec<String>) -> Literal {
        Literal::Null
    }

    pub fn expect(&mut self, tok_type: TokenType, cursor: usize, error_message: String) {
        if self.tokens[cursor].token_type == tok_type {
            return;
        } else {
            self.throw_error(self.tokens[cursor].line_num, error_message);
        }
    }

    pub fn throw_error(&mut self, line: usize, message: String) {
        self.errors
            .push(VeloError::error(line, &message, ParseError));
    }
}
