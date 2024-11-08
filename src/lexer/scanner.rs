use super::{LexerError, Literal, Token, TokenType};

struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            column: 0,
        }
    }

    fn scan_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(e),
            }
        }
        tokens.push(Token::new(TokenType::EOF, "", Literal::Nil, self.line));
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let c = self.advance();
        match c {
            '(' => Ok(self.get_token(TokenType::LeftParen)),
            ')' => Ok(self.get_token(TokenType::RightParen)),
            '{' => Ok(self.get_token(TokenType::LeftBrace)),
            '}' => Ok(self.get_token(TokenType::RightBrace)),
            ',' => Ok(self.get_token(TokenType::Comma)),
            '.' => Ok(self.get_token(TokenType::Dot)),
            '-' => Ok(self.get_token(TokenType::Minus)),
            '+' => Ok(self.get_token(TokenType::Plus)),
            ';' => Ok(self.get_token(TokenType::Semicolon)),
            '*' => Ok(self.get_token(TokenType::Star)),
            _ => Err(LexerError::new("unknown token type", self.line, self.column)),
        }
    }

    fn get_token(&self ,token_type: TokenType) -> Token {
        Token::new(token_type, "", Literal::Nil, self.line)
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.column += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, LexerError> {
    Scanner::new(source.to_string()).scan_tokens()
}
