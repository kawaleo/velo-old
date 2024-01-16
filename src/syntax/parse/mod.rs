#[allow(unused)]
mod stmt;

use super::ast::Expression;
use super::ast::*;
use super::error::{ErrorType::ParseError, VeloError, ERROR_INDICATOR};
use super::lexer::{KeywordMap, Token, TokenType, Type, KEYWORDS};

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
                    self.variable_assignment(false, None, false);
                }
                TokenType::Const => {
                    self.variable_assignment(false, None, true);
                }
                TokenType::Identifier => match self.tokens[1].token_type {
                    TokenType::DoubleColon => self.function_declaration(),
                    TokenType::ColonEq => {
                        self.variable_assignment(false, None, false);
                    }
                    TokenType::Eq => unimplemented!(), // for not reassignment
                    _ => unimplemented!(),
                },
                TokenType::Semicolon => {
                    self.tokens.remove(0);
                }
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
                    ParseError => println!("This error is found to be of type 'ParseError'"),
                    _ => unreachable!(),
                }
            }
            process::exit(1);
        }

        Ok(ast_nodes)
    }

    fn parse_literal(
        &mut self,
        token: Token,
        expected: &Type,
        infer_type: bool,
    ) -> (Expression, Option<Type>) {
        match token.token_type {
            TokenType::True => {
                if expected != &Type::Bool && !infer_type {
                    let message = format!(
                        "{} \x1b[1mExpected type '{}' but found 'bool'",
                        ERROR_INDICATOR,
                        Type::to_string(&expected)
                    );
                    self.throw_error(token.line_num, message);
                    (Expression::Literal(Literal::Bool(true)), None)
                } else {
                    (Expression::Literal(Literal::Bool(true)), Some(Type::Bool))
                }
            }
            TokenType::False => {
                if expected != &Type::Bool && !infer_type {
                    let message = format!(
                        "{} \x1b[1mExpected type '{}' but found 'bool'",
                        ERROR_INDICATOR,
                        Type::to_string(&expected)
                    );
                    self.throw_error(token.line_num, message);
                    (Expression::Literal(Literal::Bool(false)), None)
                } else {
                    (Expression::Literal(Literal::Bool(false)), Some(Type::Bool))
                }
            }
            TokenType::String => (
                Expression::Literal(Literal::StringLiteral(token.lexeme.clone())),
                Some(Type::String),
            ),
            TokenType::NumericLiteral => {
                let has_operator = self.tokens.get(self.cursor + 1).map_or(false, |t| {
                    matches!(
                        t.token_type,
                        TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div
                    )
                });

                let message = format!(
                    "{} \x1b[1mFailed to parse '{}' as a numeric literal\x1b[0m",
                    ERROR_INDICATOR, token.lexeme,
                );

                let is_f32 = token.lexeme.contains('.');

                let v = token
                    .lexeme
                    .parse::<f32>()
                    .map_err(|_| self.throw_error(self.tokens[0].line_num, message))
                    .unwrap();

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
                        false => {
                            let res =
                                Self::evaluate_expression(&to_eval, &expected, &token, infer_type);
                            if res.1.is_none() {
                                (res.0, Some(Type::Float))
                            } else {
                                self.throw_error(self.tokens[0].line_num, res.1.unwrap());
                                return (
                                    Expression::Literal(Literal::Float(0.0)),
                                    Some(Type::Float),
                                );
                            }
                        }
                        _ => {
                            self.throw_error(self.tokens[1].line_num, keyword_error_msg);
                            return (Expression::Literal(Literal::Float(0.0)), Some(Type::Float));
                        }
                    }
                } else {
                    if !infer_type {
                        match expected {
                            Type::Short if v as i16 as f32 == v => {
                                (Expression::Literal(Literal::Short(v as i16)), None)
                            }
                            Type::Int if v as i32 as f32 == v => {
                                (Expression::Literal(Literal::Int(v as i32)), None)
                            }
                            Type::Large if v as i64 as f32 == v => {
                                (Expression::Literal(Literal::Large(v as i64)), None)
                            }
                            Type::Float => (Expression::Literal(Literal::Float(v)), None),
                            _ => {
                                let message = format!(
                                    "{} Expected '{:#?}' found 'int'",
                                    ERROR_INDICATOR, token.token_type
                                );

                                self.throw_error(self.tokens[0].line_num, message);

                                (Expression::Literal(Literal::Float(v)), Some(Type::Float))
                            }
                        }
                    } else {
                        if !is_f32 {
                            match v {
                                // Check if it's a float and return Float if so
                                val if val.is_sign_positive() && val.fract() != 0.0 => {
                                    (Expression::Literal(Literal::Float(val)), Some(Type::Float))
                                }
                                // Check if it's a whole number and fits into i16
                                val if val.fract() == 0.0 && (val as i16 as f32 == val) => (
                                    Expression::Literal(Literal::Short(val as i16)),
                                    Some(Type::Short),
                                ),
                                // Check if it's a whole number and fits into i32
                                val if val.fract() == 0.0 && (val as i32 as f32 == val) => (
                                    Expression::Literal(Literal::Int(val as i32)),
                                    Some(Type::Int),
                                ),
                                // For values larger than i32 or with decimal parts, use i64 (Large)
                                val if val.fract() == 0.0 && (val as i64 as f32 == val) => (
                                    Expression::Literal(Literal::Large(val as i64)),
                                    Some(Type::Large),
                                ),
                                _ => (Expression::Literal(Literal::Float(v)), Some(Type::Float)), // todo: throw error
                            }
                        } else {
                            (Expression::Literal(Literal::Float(v)), Some(Type::Float))
                        }
                    }
                }
            }
            _ => {
                let message = format!(
                    "{} \x1b[1mCannot assign items of type {:#?} to variables\x1b[0m",
                    ERROR_INDICATOR, token.token_type
                );
                self.throw_error(self.tokens[0].line_num, message);
                self.cursor = 0;

                (Expression::Literal(Literal::Null), Some(Type::Void))
            }
        }
    }

    fn evaluate_expression(
        expr: &[String],
        expected: &Type,
        token: &Token,
        infer_type: bool,
    ) -> (Expression, Option<String>, Option<Type>) {
        let mut nums: Vec<f32> = Vec::new();
        let mut ops: Vec<&str> = Vec::new();

        let mut i = 0;
        while i < expr.len() {
            match expr[i].as_str() {
                "+" | "-" | "*" | "/" => {
                    ops.push(&expr[i]);
                }
                _ => {
                    let num = expr[i].parse::<f32>();
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

        let message_wrong_type = format!(
            "{} \x1b[1mExpected '{:#?}' found 'int'\x1b[0m",
            ERROR_INDICATOR, token.token_type
        );

        if !infer_type {
            let message = match expected {
                Type::Short if (result as i16 as f32) != result => Some(format!(
                    "{} \x1b[1mLoss of precision: 'float' value is out of range for 'short'",
                    ERROR_INDICATOR
                )),
                Type::Int if (result as i32 as f32) != result => Some(format!(
                    "{} \x1b[1mLoss of precision: 'float' value is out of range for 'int'",
                    ERROR_INDICATOR
                )),
                Type::Large if (result as i64 as f32) != result => Some(format!(
                    "{} \x1b[1mLoss of precision: 'float' value is out of range for 'large'",
                    ERROR_INDICATOR
                )),
                _ => None,
            };

            match expected {
                Type::Short => (
                    Expression::Literal(Literal::Short(result as i16)),
                    message,
                    None,
                ),
                Type::Int => (
                    Expression::Literal(Literal::Int(result as i32)),
                    message,
                    None,
                ),
                Type::Large => (
                    Expression::Literal(Literal::Large(result as i64)),
                    message,
                    None,
                ),
                Type::Float => (Expression::Literal(Literal::Float(result)), None, None),
                _ => (
                    Expression::Literal(Literal::Float(result)),
                    Some(message_wrong_type),
                    None,
                ),
            }
        } else {
            match result {
                // Check if it's a float and return Float if so
                val if val.is_sign_positive() && val.fract() != 0.0 => (
                    Expression::Literal(Literal::Float(val)),
                    None,
                    Some(Type::Float),
                ),
                // Check if it's a whole number and fits into i16
                val if val.fract() == 0.0 && (val as i16 as f32 == val) => (
                    Expression::Literal(Literal::Short(val as i16)),
                    None,
                    Some(Type::Short),
                ),
                // Check if it's a whole number and fits into i32
                val if val.fract() == 0.0 && (val as i32 as f32 == val) => (
                    Expression::Literal(Literal::Int(val as i32)),
                    None,
                    Some(Type::Int),
                ),
                // For values larger than i32 or with decimal parts, use i64 (Large)
                val if val.fract() == 0.0 && (val as i64 as f32 == val) => (
                    Expression::Literal(Literal::Large(result as i64)),
                    None,
                    Some(Type::Large),
                ),
                _ => (
                    Expression::Literal(Literal::Float(result)),
                    Some(message_wrong_type),
                    None,
                ),
            }
        }
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
