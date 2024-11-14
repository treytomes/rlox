use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub struct LexerError {
    pub msg: String,
    pub line: usize,
    pub column: usize,
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
