use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::{debug::FileLocation, lexer::Token};

pub struct ParserError {
    pub msg: String,
    line: usize,
    column: usize,
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
            line: token.get_line(),
            column: token.get_column(),
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

impl FileLocation for ParserError {
    fn get_line(&self) -> usize {
        self.line
    }

    fn get_column(&self) -> usize {
        self.column
    }
}

impl Error for ParserError {}

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
