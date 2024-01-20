#[allow(unused)]
mod stmt;

use super::ast::Expression;
use super::ast::*;
use super::lexer::{KeywordMap, Token, TokenType, Type, KEYWORDS};
use crate::error::{ErrorType::ParseError, VeloError, ERROR_INDICATOR};

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
                TokenType::Import => {
                    self.parse_import();
                }
                TokenType::Immut => {
                    self.variable_assignment(false, None, true, false);
                }
                TokenType::Function => self.function_declaration(),
                TokenType::Identifier => {
                    match self.tokens[1].token_type {
                        TokenType::ColonEq => {
                            self.variable_assignment(false, None, false, false);
                        }
                        TokenType::LParen => {
                            self.call_expr();
                        }
                        TokenType::Eq => unimplemented!(), // for not reassignment
                        _ => unimplemented!(),
                    }
                }
                TokenType::Semicolon => {
                    println!("Itsa semicolon");
                    self.tokens.remove(0);
                }
                TokenType::EOF => {
                    self.nodes.push(Ast::Expression(Expression::Null));
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

    pub fn parse_import(&mut self) {
        self.cursor += 1; // `import`

        if let Some(import_path) = self.tokens.get(self.cursor) {
            if import_path.token_type != TokenType::String {
                println!(
                    "expected string for import path but found '{}'",
                    TokenType::to_string(import_path.token_type)
                );
                process::exit(1);
            }
            self.cursor += 1;
            let is_library = vec!["std/io, std"].contains(&import_path.lexeme.as_str());

            let node = Ast::Statement(Statement::ImportPath {
                path: import_path.lexeme.clone(),
                is_library,
            });

            self.nodes.push(node)
        } else {
            println!("unexpected EOF");
            process::exit(1)
        }

        self.tokens.drain(0..self.cursor);
    }

    pub fn call_expr(&mut self) {
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
                    let literal = self.parse_literal(
                        param_token.clone(),
                        &Type::Void,
                        true,
                        Some(self.cursor),
                    );
                    self.cursor += 1;

                    params.push(literal.0);
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

        self.tokens.drain(0..self.cursor); // so uhh... forgot to add this line...
                                           // took 2 hours to figure out why it wasnt working
                                           // having fun :)

        self.cursor = 0; // once again, forgot to add this line
                         // took me around 30 minutes before i walked away
                         // literally figured out the error while rock climbing... lol
        self.nodes.push(Ast::Expression(call_expr));
    }

    fn parse_literal(
        &mut self,
        token: Token,
        expected: &Type,
        infer_type: bool,
        cursor: Option<usize>,
    ) -> (Expression, Option<Type>) {
        match cursor.is_some() {
            true => self.cursor = cursor.unwrap(),
            _ => {}
        }
        match token.token_type {
            TokenType::True => {
                if expected != &Type::Bool && !infer_type {
                    let message = format!(
                        "{} \x1b[1mExpected type '{}' but found 'bool'",
                        ERROR_INDICATOR,
                        Type::to_string(&expected)
                    );
                    self.throw_error(token.line_num, message);
                    (Expression::Bool(true), None)
                } else {
                    (Expression::Bool(true), Some(Type::Bool))
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
                    (Expression::Bool(false), None)
                } else {
                    (Expression::Bool(false), Some(Type::Bool))
                }
            }
            TokenType::String => (
                Expression::StringLiteral(token.lexeme.clone()),
                Some(Type::String),
            ),
            TokenType::NumericLiteral | TokenType::Identifier => {
                let mut to_eval: Vec<Token> = Vec::new();

                let mut keyword_error = false;
                let mut keyword_fault = TokenType::Null;
                let mut current_index = self.cursor + 1;

                to_eval.push(self.tokens[current_index - 1].clone());

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
                            to_eval.push(next_token.clone());
                            current_index += 1;
                        } else {
                            to_eval.push(next_token.clone());
                            current_index += 1;
                        }
                    } else {
                        break;
                    }
                }

                /*
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
                */

                let keyword_error_msg = format!(
                    "{} \x1b[1mExpected ';' after expression, found keyword '{}'\x1b[0m",
                    ERROR_INDICATOR,
                    TokenType::to_string(keyword_fault),
                );

                self.cursor = current_index - 1;

                match keyword_error {
                    false => {
                        let res = Self::parse_expression(to_eval);
                        (res, Some(Type::Float))
                    }
                    _ => {
                        self.throw_error(self.tokens[1].line_num, keyword_error_msg);
                        return (Expression::Float(0.0), Some(Type::Float));
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

                (Expression::Null, Some(Type::Void))
            }
        }
    }

    fn parse_expression(tokens: Vec<Token>) -> Expression {
        let mut nums: Vec<Expression> = Vec::new();
        let mut ops: Vec<TokenType> = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            match tokens[i].token_type {
                TokenType::Add | TokenType::Sub | TokenType::Mul | TokenType::Div => {
                    ops.push(tokens[i].token_type.clone())
                }
                TokenType::Identifier => {
                    let num = tokens[i].lexeme.clone();
                    nums.push(Expression::Identifier(num))
                }
                _ => {
                    let num = tokens[i].lexeme.clone().parse::<f32>();
                    if let Ok(value) = num {
                        nums.push(Expression::Float(num.unwrap()))
                    }
                }
            }
            i += 1;
        }

        let mut result_expr = nums.pop().unwrap();
        while let Some(op) = ops.pop() {
            let rhs_expr = nums.pop().unwrap();
            result_expr = Expression::BinaryOp {
                lhs: Box::new(Ast::Expression(result_expr)),
                op,
                rhs: Box::new(Ast::Expression(rhs_expr)),
            };
        }

        result_expr
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

    pub fn throw_error_ref(&mut self, _line: &usize, message: String) {
        self.errors.push(VeloError::error(0, &message, ParseError));
        // FIXME: In a huge war with the borrow checker rn (ima win)
    }
}
