/**
 * The token parser for this language grammar:
 * program        → statement* EOF ;
 * statement      → exprStmt
 *                | letStmt
 *                | printStmt ;
 * letStmt        → "let" IDENTIFIER ( "=" expression )? ";" ;
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
    debug::{ErrorSet, FileLocation, HasFileLocation},
    lexer::{Token, TokenType},
};

use super::{BinaryOp, Expr, ParserError, TokenStream, UnaryOp};

pub fn parse(tokens: &Vec<Token>) -> Result<Expr, ErrorSet> {
    let mut stream = TokenStream::new(tokens.clone());
    if stream.is_at_end() {
        let mut errors = ErrorSet::new();
        errors.push(ParserError::new("unexpected end of file", 1, 1));
        return Err(errors);
    }

    let expr = parse_program(&mut stream)?;
    Ok(expr)
}

fn parse_program(stream: &mut TokenStream) -> Result<Expr, ErrorSet> {
    let loc = FileLocation::from_loc(stream.peek().unwrap());
    let mut exprs = Vec::new();
    let mut errors = ErrorSet::new();
    while !stream.is_at_end() {
        match parse_stmt(stream) {
            Ok(expr) => exprs.push(expr),
            Err(e) => {
                errors.push(e);
                synchronize(stream);
                continue;
            }
        };

        if let Some(token) = stream.peek() {
            // The last statement need not end with a semicolon.
            if !vec![TokenType::Comma, TokenType::Semicolon].contains(&token.token_type) {
                break;
            }
            match stream.consume(vec![TokenType::Comma, TokenType::Semicolon]) {
                Ok(_) => continue,
                Err(e) => {
                    errors.push(e);
                    synchronize(stream);
                    continue;
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(Expr::program(&loc, exprs))
    } else {
        Err(errors)
    }
}

fn parse_stmt(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    if let Some(token) = stream.peek() {
        match token.token_type {
            TokenType::Print => parse_stmt_print(stream),
            TokenType::Let => parse_stmt_let(stream),
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

fn parse_stmt_expr(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let expr = parse_expr(stream)?;
    Ok(expr)
}

fn parse_stmt_let(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let loc = FileLocation::from_loc(stream.peek().unwrap());
    stream.consume(vec![TokenType::Let])?;
    let name = stream.consume(vec![TokenType::Identifier])?;
    let initializer = if stream.match_token(vec![TokenType::Equal]) {
        Some(parse_expr(stream)?)
    } else {
        None
    };
    Ok(Expr::let_stmt(&loc, name.lexeme.clone(), initializer))
}

fn parse_stmt_print(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let loc = FileLocation::from_loc(stream.peek().unwrap());
    stream.consume(vec![TokenType::Print])?;
    let expr = parse_expr(stream)?;
    Ok(Expr::print(&loc, expr))
}

fn parse_expr(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    parse_assignment(stream)
}

fn parse_assignment(stream: &mut TokenStream) -> Result<Expr, ParserError> {
    let loc = FileLocation::from_loc(stream.peek().unwrap());
    let expr = parse_equality(stream)?;
    if stream.match_token(vec![TokenType::Equal]) {
        let value = parse_assignment(stream)?;
        match expr {
            Expr::Variable(_, name) => Ok(Expr::assign(&loc, name, value)),
            _ => Err(ParserError::new(
                "invalid assignment target",
                loc.get_line(),
                loc.get_column(),
            )),
        }
    } else {
        Ok(expr)
    }
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
            | TokenType::String
            | TokenType::Identifier => Ok(Expr::literal(&loc, token.literal.clone())),
            TokenType::LeftParen => {
                let expr = parse_expr(stream)?;
                stream.consume(vec![TokenType::RightParen])?;
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
        if vec![TokenType::Comma, TokenType::Semicolon].contains(&token.token_type) {
            return;
        }

        match token.token_type {
            TokenType::Class
            | TokenType::Fun
            | TokenType::Let
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
