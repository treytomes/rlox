use std::{
    fmt::{Binary, Display},
    str::FromStr,
};

use crate::lexer::{Token, TokenType};

use super::ParserError;

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LogicalAnd,
    LogicalOr,
}

impl BinaryOp {
    pub fn from_token(token: &Token) -> Result<Self, ParserError> {
        match token.token_type {
            TokenType::Plus => Ok(BinaryOp::Add),
            TokenType::Minus => Ok(BinaryOp::Sub),
            TokenType::Star => Ok(BinaryOp::Mul),
            TokenType::Slash => Ok(BinaryOp::Div),
            TokenType::EqualEqual => Ok(BinaryOp::Eq),
            TokenType::BangEqual => Ok(BinaryOp::Ne),
            TokenType::Less => Ok(BinaryOp::Lt),
            TokenType::LessEqual => Ok(BinaryOp::Le),
            TokenType::Greater => Ok(BinaryOp::Gt),
            TokenType::GreaterEqual => Ok(BinaryOp::Ge),
            TokenType::LogicalAnd => Ok(BinaryOp::LogicalAnd),
            TokenType::LogicalOr => Ok(BinaryOp::LogicalOr),
            _ => Err(ParserError::unexpected_token(token)),
        }
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Sub => write!(f, "-"),
            BinaryOp::Mul => write!(f, "*"),
            BinaryOp::Div => write!(f, "/"),
            BinaryOp::Eq => write!(f, "=="),
            BinaryOp::Ne => write!(f, "!="),
            BinaryOp::Lt => write!(f, "<"),
            BinaryOp::Le => write!(f, "<="),
            BinaryOp::Gt => write!(f, ">"),
            BinaryOp::Ge => write!(f, ">="),
            BinaryOp::LogicalAnd => write!(f, "&&"),
            BinaryOp::LogicalOr => write!(f, "||"),
        }
    }
}

impl FromStr for BinaryOp {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(BinaryOp::Add),
            "-" => Ok(BinaryOp::Sub),
            "*" => Ok(BinaryOp::Mul),
            "/" => Ok(BinaryOp::Div),
            "==" => Ok(BinaryOp::Eq),
            "!=" => Ok(BinaryOp::Ne),
            "<" => Ok(BinaryOp::Lt),
            "<=" => Ok(BinaryOp::Le),
            ">" => Ok(BinaryOp::Gt),
            ">=" => Ok(BinaryOp::Ge),
            "&&" => Ok(BinaryOp::LogicalAnd),
            "||" => Ok(BinaryOp::LogicalOr),
            _ => Err(ParserError::invalid_op(s)),
        }
    }
}
