use super::{LexerError, Literal, Token, TokenType};

struct Scanner {
    source: String,
    start: usize,

    // The index into the source string.
    current: usize,

    // The current line.
    line: usize,

    // The current column.
    column: usize,

    // This is the output.
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) -> Result<(), LexerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            "",
            Literal::Nil,
            self.line,
            self.column,
        ));
        Ok(())
    }

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let c = self.advance();
        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            ';' => Ok(self.add_token(TokenType::Semicolon)),
            '*' => Ok(self.add_token(TokenType::Star)),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                Ok(self.add_token(token_type))
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                Ok(self.add_token(token_type))
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                Ok(self.add_token(token_type))
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                Ok(self.add_token(token_type))
            }
            '/' => {
                if self.match_next('/') {
                    self.line_comment()
                } else if self.match_next('*') {
                    self.block_comment()
                } else {
                    Ok(self.add_token(TokenType::Slash))
                }
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            ' ' | '\t' => self.whitespace(),
            '\r' | '\n' => self.newline(),
            _ => Err(LexerError::new(
                "unknown token type",
                self.line,
                self.column,
            )),
        }
    }

    fn block_comment(&mut self) -> Result<(), LexerError> {
        let mut depth = 1;
        while depth > 0 && !self.is_at_end() {
            if self.peek() == '/' && self.peek_next() == '*' {
                // Skip over nested block comments, and increase our nesting level.
                depth += 1;
                self.advance();
                self.advance();
            } else if self.peek() == '*' && self.peek_next() == '/' {
                // Pop up one nesting level.
                depth -= 1; // If this moves the depth to 0 then the loop is done.
                self.advance();
                self.advance();
            } else if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
                self.advance();
            } else {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            TokenType::Comment,
            value,
            Literal::Nil,
            self.line,
            self.column,
        ));
        Ok(())
    }

    fn line_comment(&mut self) -> Result<(), LexerError> {
        while self.peek() != '\n' && !self.is_at_end() {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            TokenType::Comment,
            value,
            Literal::Nil,
            self.line,
            self.column,
        ));
        Ok(())
    }

    fn newline(&mut self) -> Result<(), LexerError> {
        // If peek isn't \n, then the current char should be \n.  The next one might still be \n.

        if self.peek() == '\n' {
            self.tokens.push(Token::new(
                TokenType::NewLine,
                "\r\n",
                Literal::Nil,
                self.line,
                self.column,
            ));
            self.advance();
        } else {
            self.tokens.push(Token::new(
                TokenType::NewLine,
                "\n",
                Literal::Nil,
                self.line,
                self.column,
            ));
        }
        self.line += 1;
        self.column = 0;
        Ok(())
    }

    fn whitespace(&mut self) -> Result<(), LexerError> {
        while self.is_space(self.peek()) {
            self.advance();
        }

        let value = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            TokenType::Whitespace,
            value,
            Literal::Nil,
            self.line,
            self.column,
        ));
        Ok(())
    }

    fn string(&mut self) -> Result<(), LexerError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            if self.peek() == '\\' {
                // Skip over the escape character.
                self.advance();
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(LexerError::new(
                "unterminated string",
                self.line,
                self.column,
            ));
        }

        // The closing ".
        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        // Trim the surrounding quotes.
        let value = value
            .replace("\\t", "\t")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\\\", "\\")
            .replace("\\", "\"");

        self.tokens.push(Token::new(
            TokenType::String,
            value.as_str(),
            Literal::String(value.to_string()),
            self.line,
            self.column,
        ));
        Ok(())
    }

    fn number(&mut self) -> Result<(), LexerError> {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            TokenType::Number,
            value,
            Literal::Number(value.parse().unwrap()),
            self.line,
            self.column,
        ));
        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LexerError> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };

        match token_type {
            TokenType::True => {
                self.tokens.push(Token::new(
                    TokenType::True,
                    "true",
                    Literal::Boolean(true),
                    self.line,
                    self.column,
                ));
            }
            TokenType::False => {
                self.tokens.push(Token::new(
                    TokenType::False,
                    "false",
                    Literal::Boolean(false),
                    self.line,
                    self.column,
                ));
            }
            _ => {
                self.tokens.push(Token::new(
                    token_type,
                    text,
                    Literal::Identifier(text.to_string()),
                    self.line,
                    self.column,
                ));
            }
        }
        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            "",
            Literal::Nil,
            self.line,
            self.column,
        ))
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        self.column += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        self.column += 1;
        true
    }

    fn is_space(&self, c: char) -> bool {
        c == ' ' || c == '\t'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, LexerError> {
    let mut scanner = Scanner::new(source.to_string());
    scanner.scan_tokens()?;
    Ok(scanner.tokens)
}
