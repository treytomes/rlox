use crate::lexer::Literal;

use super::{BinaryOp, UnaryOp, Visitor};

pub enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Grouping(Box<Expr>),
    // Variable(String),
    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
}

impl Expr {
    pub fn number(n: f64) -> Self {
        Self::Number(n)
    }

    pub fn string(s: String) -> Self {
        Self::String(s)
    }

    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b)
    }

    pub fn literal(l: Literal) -> Self {
        match l {
            Literal::Number(n) => Self::number(n),
            Literal::String(s) => Self::string(s),
            Literal::Boolean(b) => Self::boolean(b),
            Literal::Nil => Self::Nil,
            Literal::Identifier(_) => todo!(),
        }
    }

    pub fn nil() -> Self {
        Self::Nil
    }

    pub fn grouping(e: Expr) -> Self {
        Self::Grouping(Box::new(e))
    }

    pub fn unary_op(op: UnaryOp, e: Expr) -> Self {
        Self::UnaryOp(op, Box::new(e))
    }

    pub fn binary_op(e1: Expr, op: BinaryOp, e2: Expr) -> Self {
        Self::BinaryOp(Box::new(e1), op, Box::new(e2))
    }

    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Self::Number(n) => visitor.visit_number(n),
            Self::String(s) => visitor.visit_string(s),
            Self::Boolean(b) => visitor.visit_boolean(b),
            Self::Nil => visitor.visit_nil(),
            Self::Grouping(e) => visitor.visit_grouping(e),
            Self::UnaryOp(op, e) => visitor.visit_unary_op(op, e),
            Self::BinaryOp(op, e1, e2) => visitor.visit_binary_op(e1, op, e2),
        }
    }
}
