use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Error,
    Eof,
    Plus,
    Minus,
    Star,
    Slash,
    Number,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Span,
    pub value: Option<TokenValue<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue<'a> {
    Number(f64),
    String(&'a str),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    start: usize,
    end: usize,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, start: usize, end: usize, value: Option<TokenValue<'a>>) -> Self {
        Token {
            kind,
            span: Span::new(start, end),
            value,
        }
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenKind::Error => "Error",
            TokenKind::Eof => "Eof",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Number => "Number",
        };

        write!(f, "{}", value)
    }
}

impl<'a> Display for TokenValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self)
    }
}
