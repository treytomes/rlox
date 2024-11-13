use super::{Literal, TokenType};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub literal: Literal,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Literal,
        line: usize,
        column: usize,
    ) -> Token {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
            column,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
