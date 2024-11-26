use std::{
    error::Error,
    fmt::{Debug, Display},
};

use crate::debug::HasFileLocation;

/**
 * Indicates that the interpreter should stop executing code.
 *
 * If the interrupt makes it all the way to the top of the program the runtime error will be thrown to the user.
 */
#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    // Indicates that a loop should be broken out of.
    Break,

    // Indicates that the remaining code in the current scope should be skipped.
    Continue,
}

impl Display for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interrupt::Break => write!(f, "break"),
            Interrupt::Continue => write!(f, "continue"),
        }
    }
}

pub struct RuntimeError {
    pub msg: String,
    line: usize,
    column: usize,
    pub interrupt: Option<Interrupt>,
}

impl RuntimeError {
    pub fn new(msg: &str, line: usize, column: usize) -> Self {
        Self {
            msg: msg.to_string(),
            line,
            column,
            interrupt: None,
        }
    }

    pub fn break_loop() -> Self {
        Self {
            msg: "break outside of a loop".to_string(),
            line: 0,
            column: 0,
            interrupt: Some(Interrupt::Break),
        }
    }

    pub fn continue_loop() -> Self {
        Self {
            msg: "continue outside of a loop".to_string(),
            line: 0,
            column: 0,
            interrupt: Some(Interrupt::Continue),
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
