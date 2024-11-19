use std::{error::Error, fmt::Display};

use crate::{
    expr::{Binary, Expr, Literal, LiteralValue},
    lexer::Lexer,
    token::{
        Token,
        TokenKind::{self, *},
        TokenValue,
    },
};

type Result<T, E = ParserError> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    src: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            src,
            lexer: Lexer::new(src),
        }
    }

    fn parse(&mut self) -> Result<Expr> {
        self.expr()
    }

    // fn expr(&mut self) -> Result<Expr> {
    //     let mut expr = self.factor()?;

    //     while let Some(token) = self.peek() {
    //         match token.kind {
    //             Plus | Minus => {
    //                 let op = self.advance().unwrap();
    //                 let right = self.primary()?;
    //                 expr = Expr::Binary(Binary::new(expr, op, right));
    //             }
    //             _ => break,
    //         }
    //     }

    //     Ok(expr)
    // }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        while {
            let kind = self.peek().map(|t| t.kind);
            matches!(kind, Some(Star | Slash))
        } {
            let op = self.advance().unwrap();
            let right = self.primary()?;
            expr = Expr::Binary(Binary::new(expr, op, right));
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expr> {
        let current = self.advance().unwrap();

        match current.kind {
            Number => {
                if let TokenValue::Number(value) = current.value.unwrap() {
                    return Ok(Expr::Literal(Literal::new(LiteralValue::Number(value))));
                } else {
                    Err(ParserError::InvalidToken)
                }
            }
            _ => Err(ParserError::InvalidToken),
        }
    }

    fn advance(&mut self) -> Option<Token<'a>> {
        self.lexer.next()
    }

    fn peek(&self) -> Option<Token<'a>> {
        self.lexer.clone().next()
    }

    fn next_if_matches(&mut self, kinds: &[TokenKind]) -> Option<Token<'a>> {
        if let Some(token) = self.peek() {
            if kinds.contains(&token.kind) {
                return self.advance();
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    ParseFloatError(std::num::ParseFloatError),
    InvalidToken,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::ParseFloatError(value) => write!(f, "Parse Error: {}", value),
            ParserError::InvalidToken => write!(f, "Unexpected token"),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_addition() {
        let mut parser = Parser::new("1+2");
        assert_eq!(parser.parse(), 3.0);
    }
}
