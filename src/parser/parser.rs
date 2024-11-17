/**
 * The token parser for this language grammar:
 * program        → statement* EOF ;
 * statement      → exprStmt
 *                | printStmt ";" ;
 * printStmt      → "print" expression ";" ;
 * exprStmt       → expression ";" ;
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
    debug::{FileLocation, HasFileLocation},
    lexer::{Token, TokenType},
};

use super::{BinaryOp, Expr, ParserError, TokenStream, UnaryOp};

pub fn parse(tokens: &Vec<Token>) -> Result<Expr, ParserError> {
    let mut stream = TokenStream::new(tokens.clone());
    if stream.is_at_end() {
        return Err(ParserError::new("unexpected end of file", 1, 1));
    }

    let expr = parse_stmt(&mut stream)?;
    Ok(expr)
}

fn parse_stmt(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    if let Some(token) = stream.next() {
        match token.token_type {
            TokenType::Print => parse_stmt_print(stream),
            _ => parse_stmt_expr(stream),
        }
    } else {
        let prev = stream.prev().unwrap();
        Err(ParserError::new(
            "unexpected end of file",
            prev.get_line(),
            prev.get_column(),
        ))
    }
}

fn parse_stmt_print(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let loc = FileLocation::from_loc(stream.peek().unwrap());
    stream.consume(TokenType::Print)?;
    let expr = parse_expr(stream)?;
    stream.consume(TokenType::Semicolon)?;
    Ok(Expr::print(&loc, expr))
}

fn parse_stmt_expr(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let expr = parse_expr(stream)?;
    stream.consume(TokenType::Semicolon)?;
    Ok(expr)
}

fn parse_expr(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    parse_equality(stream)
}

fn parse_equality(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_comparison(stream)?;

    while let Some(token) = stream.peek() {
        let loc = FileLocation::from_loc(token);
        match token.token_type {
            TokenType::BangEqual | TokenType::EqualEqual => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_comparison(stream)?;
                expr = Expr::binary_op(&loc, expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_comparison(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_term(stream)?;

    while let Some(token) = stream.peek() {
        let loc = FileLocation::from_loc(token);
        match token.token_type {
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_term(stream)?;
                expr = Expr::binary_op(&loc, expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_term(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_factor(stream)?;

    while let Some(token) = stream.peek() {
        let loc = FileLocation::from_loc(token);
        match token.token_type {
            TokenType::Minus | TokenType::Plus => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_factor(stream)?;
                expr = Expr::binary_op(&loc, expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_factor(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let mut expr = parse_unary(stream)?;

    while let Some(token) = stream.peek() {
        let loc = FileLocation::from_loc(token);
        match token.token_type {
            TokenType::Slash | TokenType::Star => {
                let operator = BinaryOp::from_token(stream.next().unwrap())?;
                let right = parse_unary(stream)?;
                expr = Expr::binary_op(&loc, expr, operator, right);
            }
            _ => break,
        }
    }

    Ok(expr)
}

fn parse_unary(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    if let Some(token) = stream.peek() {
        let loc = FileLocation::from_loc(token);
        match token.token_type {
            TokenType::Bang | TokenType::Minus => {
                let operator = UnaryOp::from_token(stream.next().unwrap())?;
                let right = parse_unary(stream)?;
                return Ok(Expr::unary_op(&loc, operator, right));
            }
            _ => {}
        }
    }

    parse_primary(stream)
}

fn parse_primary(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    if let Some(token) = stream.next() {
        let loc = FileLocation::from_loc(token);
        let line = token.get_line();
        let column = token.get_column();

        match token.token_type {
            TokenType::False
            | TokenType::True
            | TokenType::Nil
            | TokenType::Number
            | TokenType::String => Ok(Expr::literal(&loc, token.literal.clone())),
            TokenType::LeftParen => {
                let expr = parse_expr(stream)?;
                stream.consume(TokenType::RightParen)?;
                Ok(Expr::grouping(&loc, expr))
            }
            _ => Err(ParserError::new("expected expression", line, column)),
        }
    } else {
        if let Some(token) = stream.prev() {
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
