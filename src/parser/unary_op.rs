use std::{fmt::Display, str::FromStr};

use crate::lexer::{Token, TokenType};

use super::ParserError;

pub enum UnaryOp {
    Neg,
    Not,
}

impl UnaryOp {
    pub fn from_token(token: &Token) -> Result<Self, ParserError> {
        match token.token_type {
            TokenType::Minus => Ok(Self::Neg),
            TokenType::Bang => Ok(Self::Not),
            _ => Err(ParserError::unexpected_token(token)),
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Neg => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl FromStr for UnaryOp {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Neg),
            "!" => Ok(Self::Not),
            _ => Err(ParserError::invalid_op(s)),
        }
    }
}
