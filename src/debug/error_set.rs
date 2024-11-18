use std::{error::Error, fmt::Debug, fmt::Display};

use super::LocatableError;

pub struct ErrorSet {
    errors: Vec<Box<dyn LocatableError>>,
}

impl ErrorSet {
    pub fn new() -> Self {
        ErrorSet { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn push<E>(&mut self, err: E)
    where
        E: 'static + LocatableError,
    {
        self.errors.push(Box::new(err));
    }

    pub fn report(&self, input: &str) {
        for err in &self.errors {
            err.report(input);
        }
    }
}

impl Error for ErrorSet {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.errors.first().map(|err| err.as_error())
    }
}

impl Display for ErrorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "errors found: {}", self.errors.len())
    }
}

impl Debug for ErrorSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "errors found: {}", self.errors.len())
    }
}
