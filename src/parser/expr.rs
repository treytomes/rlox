use crate::{
    debug::{FileLocation, HasFileLocation},
    lexer::Literal,
};

use super::{BinaryOp, UnaryOp, Visitor};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(FileLocation, f64),
    String(FileLocation, String),
    Boolean(FileLocation, bool),
    Nil(FileLocation),
    Grouping(FileLocation, Box<Expr>),
    Variable(FileLocation, String),
    UnaryOp(FileLocation, UnaryOp, Box<Expr>),
    BinaryOp(FileLocation, Box<Expr>, BinaryOp, Box<Expr>),

    Print(FileLocation, Box<Expr>),
    If(FileLocation, Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    Program(FileLocation, Box<Vec<Expr>>),
    // TODO: Not sure that Let and LetInit need to be separate entities.
    Let(FileLocation, String),
    LetInit(FileLocation, String, Box<Expr>),
    Assign(FileLocation, String, Box<Expr>),
    Block(FileLocation, Box<Vec<Expr>>),
    While(FileLocation, Box<Expr>, Box<Expr>),
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

    pub fn variable(loc: &dyn HasFileLocation, v: String) -> Self {
        Self::Variable(FileLocation::from_loc(loc), v)
    }

    pub fn literal(loc: &dyn HasFileLocation, l: Literal) -> Self {
        match l {
            Literal::Number(n) => Self::number(loc, n),
            Literal::String(s) => Self::string(loc, s),
            Literal::Boolean(b) => Self::boolean(loc, b),
            Literal::Nil => Self::nil(loc),
            Literal::Identifier(v) => Self::variable(loc, v),
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

    pub fn print(loc: &dyn HasFileLocation, e: Expr) -> Self {
        Self::Print(FileLocation::from_loc(loc), Box::new(e))
    }

    pub fn if_stmt(
        loc: &dyn HasFileLocation,
        condition: Expr,
        then: Expr,
        else_: Option<Expr>,
    ) -> Self {
        match else_ {
            Some(e) => Self::If(
                FileLocation::from_loc(loc),
                Box::new(condition),
                Box::new(then),
                Some(Box::new(e)),
            ),
            None => Self::If(
                FileLocation::from_loc(loc),
                Box::new(condition),
                Box::new(then),
                None,
            ),
        }
    }

    pub fn let_stmt(loc: &dyn HasFileLocation, name: String, e: Option<Expr>) -> Self {
        match e {
            Some(e) => Self::LetInit(FileLocation::from_loc(loc), name, Box::new(e)),
            None => Self::Let(FileLocation::from_loc(loc), name),
        }
    }

    pub fn program(loc: &dyn HasFileLocation, exprs: Vec<Expr>) -> Self {
        Self::Program(FileLocation::from_loc(loc), Box::new(exprs))
    }

    pub fn block(loc: &dyn HasFileLocation, exprs: Vec<Expr>) -> Self {
        Self::Block(FileLocation::from_loc(loc), Box::new(exprs))
    }

    pub fn while_stmt(loc: &dyn HasFileLocation, condition: Expr, body: Expr) -> Self {
        Self::While(
            FileLocation::from_loc(loc),
            Box::new(condition),
            Box::new(body),
        )
    }

    pub fn assign(loc: &dyn HasFileLocation, name: String, e: Expr) -> Self {
        Self::Assign(FileLocation::from_loc(loc), name, Box::new(e))
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
            Self::Print(loc, e) => visitor.visit_print(loc, e),
            Self::If(loc, c, t, e) => visitor.visit_if(loc, c, t, e),
            Self::Let(loc, name) => visitor.visit_let(loc, name),
            Self::LetInit(loc, name, e) => visitor.visit_let_init(loc, name, e),
            Self::Assign(loc, name, e) => visitor.visit_assign(loc, name, e),
            Self::Variable(loc, name) => visitor.visit_variable(loc, name),
            Self::Program(loc, e) => visitor.visit_program(loc, e),
            Self::Block(loc, e) => visitor.visit_block(loc, e),
            Self::While(loc, c, e) => visitor.visit_while(loc, c, e),
        }
    }
}
