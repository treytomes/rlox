use crate::{
    debug::{FileLocation, HasFileLocation},
    lexer::Literal,
};

use super::{BinaryOp, UnaryOp, Visitor};

pub enum Expr {
    Number(FileLocation, f64),
    String(FileLocation, String),
    Boolean(FileLocation, bool),
    Nil(FileLocation),
    Grouping(FileLocation, Box<Expr>),
    // Variable(String),
    UnaryOp(FileLocation, UnaryOp, Box<Expr>),
    BinaryOp(FileLocation, Box<Expr>, BinaryOp, Box<Expr>),
}

impl Expr {
    pub fn number(loc: &dyn HasFileLocation, n: f64) -> Self {
        Self::Number(FileLocation::from_loc(loc), n)
    }

    pub fn string(loc: &dyn HasFileLocation, s: String) -> Self {
        Self::String(FileLocation::from_loc(loc), s)
    }

    pub fn boolean(loc: &dyn HasFileLocation, b: bool) -> Self {
        Self::Boolean(FileLocation::from_loc(loc), b)
    }

    pub fn literal(loc: &dyn HasFileLocation, l: Literal) -> Self {
        match l {
            Literal::Number(n) => Self::number(loc, n),
            Literal::String(s) => Self::string(loc, s),
            Literal::Boolean(b) => Self::boolean(loc, b),
            Literal::Nil => Self::nil(loc),
            Literal::Identifier(_) => todo!(),
        }
    }

    pub fn nil(loc: &dyn HasFileLocation) -> Self {
        Self::Nil(FileLocation::from_loc(loc))
    }

    pub fn grouping(loc: &dyn HasFileLocation, e: Expr) -> Self {
        Self::Grouping(FileLocation::from_loc(loc), Box::new(e))
    }

    pub fn unary_op(loc: &dyn HasFileLocation, op: UnaryOp, e: Expr) -> Self {
        Self::UnaryOp(FileLocation::from_loc(loc), op, Box::new(e))
    }

    pub fn binary_op(loc: &dyn HasFileLocation, e1: Expr, op: BinaryOp, e2: Expr) -> Self {
        Self::BinaryOp(FileLocation::from_loc(loc), Box::new(e1), op, Box::new(e2))
    }

    pub fn accept<R>(&self, visitor: &mut dyn Visitor<R>) -> R {
        match self {
            Self::Number(loc, n) => visitor.visit_number(loc, n),
            Self::String(loc, s) => visitor.visit_string(loc, s),
            Self::Boolean(loc, b) => visitor.visit_boolean(loc, b),
            Self::Nil(loc) => visitor.visit_nil(loc),
            Self::Grouping(loc, e) => visitor.visit_grouping(loc, e),
            Self::UnaryOp(loc, op, e) => visitor.visit_unary_op(loc, op, e),
            Self::BinaryOp(loc, op, e1, e2) => visitor.visit_binary_op(loc, e1, op, e2),
        }
    }
}
