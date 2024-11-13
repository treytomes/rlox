use std::fmt::{Debug, Display};

use crate::lexer::Token;

pub struct ParserError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
}

impl ParserError {
    pub fn new(msg: &str, line: usize, column: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line,
            column,
        }
    }

    pub fn unexpected_token(token: &Token) -> Self {
        Self {
            msg: format!("unexpected token: {}", token.token_type),
            line: token.line,
            column: token.column,
        }
    }

    pub fn invalid_op(op: &str) -> Self {
        Self {
            msg: format!("invalid operator: {}", op),
            line: 0,
            column: 0,
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Debug for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
