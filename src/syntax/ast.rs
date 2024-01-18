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
        constant: bool,
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
    ImportPath {
        path: String,
        is_library: bool,
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
    Short(i16),
    Int(i32),
    Large(i64),
    Float(f32),
    Bool(bool),
    StringLiteral(String),
    Identifier(String),
    Null,

    CallExpr {
        name: String,
        params: Vec<Expression>,
    },

    BinaryOp {
        lhs: Box<Ast>,
        op: TokenType,
        rhs: Box<Ast>,
    },
}
