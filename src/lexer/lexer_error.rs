use std::fmt::{Debug, Display};

pub struct LexerError {
    msg: String,
    line: usize,
    col: usize
}

impl LexerError {
    pub fn new(msg: &str, line: usize, col: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line,
            col,
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {} col {}", self.msg, self.line, self.col)
    }
}

impl Debug for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at line {} col {}", self.msg, self.line, self.col)
    }
}