use std::fmt::Display;

use crate::lexer::token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // These are not tokens, but we need to track them for error reporting.
    Whitespace,
    Comment,
    NewLine,

    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Colon,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    QuestionMark,
    DoubleQuestionMark,
    LogicalAnd,
    BitwiseAnd,
    LogicalOr,
    BitwiseOr,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    Let, // Original spec used "var" here.
    While,
    Break,
    Continue,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenType::NewLine => "NewLine",
            TokenType::Whitespace => "Whitespace",
            TokenType::Comment => "Comment",
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",
            TokenType::QuestionMark => "QuestionMark",
            TokenType::DoubleQuestionMark => "DoubleQuestionMark",
            TokenType::Colon => "Colon",
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",
            TokenType::Identifier => "Identifier",
            TokenType::String => "String",
            TokenType::Number => "Number",
            TokenType::LogicalAnd => "LogicalAnd",
            TokenType::BitwiseAnd => "BitwiseAnd",
            TokenType::LogicalOr => "LogicalOr",
            TokenType::BitwiseOr => "BitwiseOr",
            TokenType::Class => "Class",
            TokenType::Else => "Else",
            TokenType::False => "False",
            TokenType::Fun => "Fun",
            TokenType::For => "For",
            TokenType::If => "If",
            TokenType::Nil => "Nil",
            TokenType::Print => "Print",
            TokenType::Return => "Return",
            TokenType::Super => "Super",
            TokenType::This => "This",
            TokenType::True => "True",
            TokenType::Let => "Let",
            TokenType::While => "While",
            TokenType::Break => "Break",
            TokenType::Continue => "Continue",
            TokenType::EOF => "EOF",
        };
        write!(f, "{}", s)
    }
}
