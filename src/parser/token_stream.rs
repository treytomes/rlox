use crate::lexer::{Token, TokenType};

pub struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.skip_space();
        self.tokens.get(self.index)
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.is_at_end() {
            return None;
        }
        self.skip_space();
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }

    /**
     * Skip any tokens that don't provide value to the output expression.
     */
    fn skip_space(&mut self) {
        self.skip_tokens(vec![
            TokenType::Whitespace,
            TokenType::NewLine,
            TokenType::Comment,
        ]);
    }

    fn skip_tokens(&mut self, token_types: Vec<TokenType>) {
        while let Some(token) = self.tokens.get(self.index) {
            if token_types.contains(&token.token_type) {
                self.index += 1
            } else {
                break;
            }
        }
    }
}
