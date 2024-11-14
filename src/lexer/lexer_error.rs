use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::debug::FileLocation;

pub struct LexerError {
    pub msg: String,
    line: usize,
    column: usize,
}

impl LexerError {
    pub fn new(msg: &str, line: usize, column: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line,
            column,
        }
    }
}

impl Error for LexerError {}

impl FileLocation for LexerError {
    fn get_line(&self) -> usize {
        self.line
    }

    fn get_column(&self) -> usize {
        self.column
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at line {} column {}",
            self.msg, self.line, self.column
        )
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at line {} column {}",
            self.msg, self.line, self.column
        )
    }
}
