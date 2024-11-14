use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::debug::HasFileLocation;

pub struct RuntimeError {
    pub msg: String,
    line: usize,
    column: usize,
}

impl RuntimeError {
    pub fn new(msg: &str, line: usize, column: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line,
            column,
        }
    }
}

impl Error for RuntimeError {}

impl HasFileLocation for RuntimeError {
    fn get_line(&self) -> usize {
        self.line
    }

    fn get_column(&self) -> usize {
        self.column
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Debug for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
