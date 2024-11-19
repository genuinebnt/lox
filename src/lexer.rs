use crate::token::{TokenKind::*, TokenValue};
use std::{fmt::Display, str::Chars};

use crate::token::{Token, TokenKind};

pub type Result<'a, T> = std::result::Result<T, LexerError>;

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    src: &'a str,
    chars: Chars<'a>,
    start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer<'a> {
        Lexer {
            src,
            chars: src.chars(),
            start: src.len(),
        }
    }

    fn next_token(&mut self) -> Result<Token<'a>> {
        self.start = self.offset();

        while let Some(c) = self.chars.next() {
            match c {
                '+' => return self.set_result(Plus, self.start, self.offset(), None),
                '-' => return self.set_result(Minus, self.start, self.offset(), None),
                '*' => return self.set_result(Star, self.start, self.offset(), None),
                '/' => return self.set_result(Slash, self.start, self.offset(), None),
                ch if ch.is_digit(10) => return self.take_number(),
                _ => unimplemented!(),
            }
        }

        self.set_result(Eof, self.start, self.offset(), None)
    }

    fn take_number(&mut self) -> Result<Token<'a>> {
        // 100.000
        while let Some(v) = self.peek() {
            if v.is_digit(10) {
                self.advance();
            } else if v == '.' {
                if self.peek_next().is_some_and(|v| v.is_digit(10)) {
                    self.advance();
                } else {
                    return Err(LexerError::IllegalCharacterAfterDecimal(v.to_string()));
                }
            } else {
                return self.set_result(
                    Number,
                    self.start,
                    self.offset(),
                    Some(TokenValue::String(&self.src[self.start..self.offset()])),
                );
            }
        }
        self.set_result(
            Number,
            self.start,
            self.offset(),
            Some(TokenValue::String(&self.src[self.start..self.offset()])),
        )
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn offset(&self) -> usize {
        self.src.len() - self.chars.as_str().len()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut chars = self.chars.clone();
        chars.next();
        chars.next()
    }

    fn set_result(
        &mut self,
        kind: TokenKind,
        start: usize,
        end: usize,
        value: Option<TokenValue<'a>>,
    ) -> Result<Token<'a>> {
        Ok(Token::new(kind, start, end, value))
    }

    pub fn tokenize_collect(&mut self) -> Vec<Token<'a>> {
        self.collect()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_token().unwrap();
        if token.kind == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}

#[derive(Debug, Clone)]
pub enum LexerError {
    ParseFloatError(std::num::ParseFloatError),
    IllegalCharacterAfterDecimal(String),
}

impl std::error::Error for LexerError {}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::ParseFloatError(err) => write!(f, "{}", err),
            LexerError::IllegalCharacterAfterDecimal(value) => {
                write!(f, "Expected Number, found {}", value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: TokenKind) {
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next().unwrap().kind, kind);
        assert_eq!(lexer.next().unwrap().value, None);
    }

    // #[test]
    // fn lex_spaces() {
    //     check("   ", Whitespace);
    // }

    #[test]
    fn lex_single_tokens() {
        let kinds = vec![("+", Plus), ("-", Minus), ("*", Star), ("/", Slash)];

        for kind in kinds {
            check(kind.0, kind.1);
        }
    }
}
