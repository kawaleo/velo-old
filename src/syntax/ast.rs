#![allow(dead_code)]
#![allow(unused_variables)]
use super::lexer::{TokenType, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Expression(Expression),
    Statement(Statement),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableAssignment {
        name: String,
        var_type: Type,
        value: Expression,
    },
    Function {
        name: String,
        params: Vec<(String, Type)>,
        body: Vec<Ast>,
        ret_type: Type,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    BinarpOp(Box<Expression>, Box<Expression>, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    NumericLiteral(i32),
    StringLiteral(String),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub lhs: Box<Ast>,
    pub operator: TokenType,
    pub rhs: Box<Ast>,
}

impl From<Literal> for Ast {
    fn from(literal: Literal) -> Self {
        match literal {
            Literal::NumericLiteral(val) => Ast::Statement(Statement::VariableAssignment {
                name: "".to_string(),
                var_type: Type::Void,
                value: Expression::Literal(Literal::Null),
            }),
            Literal::StringLiteral(val) => Ast::Statement(Statement::VariableAssignment {
                name: "".to_string(),
                var_type: Type::Void,
                value: Expression::Literal(Literal::StringLiteral(val)),
            }),
            _ => Ast::Expression(Expression::Literal(Literal::Null)),
        }
    }
}
