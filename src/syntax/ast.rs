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
        body: FunctionBody,
        ret_type: Type,
    },
    ExprStmt(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBody {
    stmts: Vec<Statement>,
    exprs: Vec<Expression>,
    block: Option<Box<FunctionBody>>,
}

impl FunctionBody {
    pub fn new(
        stmts: Vec<Statement>,
        exprs: Vec<Expression>,
        block: Option<Box<FunctionBody>>,
    ) -> FunctionBody {
        FunctionBody {
            stmts,
            exprs,
            block,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
    BinaryOp(Box<Expression>, Box<Expression>, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Short(i16),
    Int(i32),
    Large(i64),
    Float(f32),
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
            Literal::Float(val) => Ast::Statement(Statement::VariableAssignment {
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
