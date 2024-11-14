/**
 * The token parser for this language grammar:
 * expression     → equality ;
 * equality       → comparison ( ( "!=" | "==" ) comparison )* ;
 * comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
 * term           → factor ( ( "-" | "+" ) factor )* ;
 * factor         → unary ( ( "/" | "*" ) unary )* ;
 * unary          → ( "!" | "-" ) unary
 *                | primary ;
 * primary        → NUMBER | STRING | "true" | "false" | "nil"
 *                | "(" expression ")" ;
 */
use crate::{
    debug::FileLocation,
    lexer::{Token, TokenType},
};

use super::{BinaryOp, Expr, ParserError, TokenStream, UnaryOp};

pub fn parse(tokens: &Vec<Token>) -> Result<Expr, ParserError> {
    let mut stream = TokenStream::new(tokens.clone());
    let expr = parse_expr(&mut stream)?;
    Ok(expr)
}

fn parse_expr(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    parse_equality(stream)
}

fn parse_equality(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_comparison(stream)?;

    while let Some(token) = stream.peek() {
        match token.token_type {
            TokenType::BangEqual | TokenType::EqualEqual => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_comparison(stream)?;
                expr = Expr::binary_op(expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_comparison(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_term(stream)?;

    while let Some(token) = stream.peek() {
        match token.token_type {
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_term(stream)?;
                expr = Expr::binary_op(expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_term(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_factor(stream)?;

    while let Some(token) = stream.peek() {
        match token.token_type {
            TokenType::Minus | TokenType::Plus => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_factor(stream)?;
                expr = Expr::binary_op(expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_factor(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_unary(stream)?;

    while let Some(token) = stream.peek() {
        match token.token_type {
            TokenType::Slash | TokenType::Star => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_unary(stream)?;
                expr = Expr::binary_op(expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_unary(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    if let Some(token) = stream.peek() {
        match token.token_type {
            TokenType::Bang | TokenType::Minus => {
                let operator = UnaryOp::from_token(stream.next().unwrap())?;
                let right = parse_unary(stream)?;
                return Ok(Expr::unary_op(operator, right));
            }
            _ => {}
        }
    }

    parse_primary(stream)
}

fn parse_primary(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    if let Some(token) = stream.next() {
        let line = token.get_line();
        let column = token.get_column();

        match token.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number
            | TokenType::String => Ok(Expr::literal(token.literal.clone())),
            TokenType::LeftParen => {
                let expr = parse_expr(stream)?;
                consume(TokenType::RightParen, stream)?;
                Ok(Expr::grouping(expr))
            }
            _ => Err(ParserError::new("expected expression", line, column)),
        }
    } else {
        if let Some(token) = stream.current() {
            Err(ParserError::new(
                "expected expression",
                token.get_line(),
                token.get_column(),
            ))
        } else {
            Err(ParserError::new("expected expression", 0, 0))
        }
    }
}

fn consume(token_type: TokenType, stream: &mut TokenStream) -> Result<Token, ParserError> {
    if let Some(token) = stream.next() {
        if token.token_type == token_type {
            return Ok(token.clone());
        }
        return Err(ParserError::new(
            format!("expected '{:?}'", token_type).as_str(),
            token.get_line(),
            token.get_column(),
        ));
    }
    Err(ParserError::new(
        format!("expected '{:?}'", token_type).as_str(),
        0,
        0,
    ))
}

fn synchronize(stream: &mut TokenStream) {
    stream.next();

    while let Some(token) = stream.peek() {
        if token.token_type == TokenType::Semicolon {
            return;
        }

        match token.token_type {
            TokenType::Class
            | TokenType::Fun
            | TokenType::Var
            | TokenType::For
            | TokenType::If
            | TokenType::While
            | TokenType::Print
            | TokenType::Return => return,
            _ => {}
        }

        stream.next();
    }
}
