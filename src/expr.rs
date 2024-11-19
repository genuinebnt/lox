use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
    Binary(Binary<'a>),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    value: LiteralValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub op: Token<'a>,
    pub right: Box<Expr<'a>>,
}

impl<'a> Binary<'a> {
    pub fn new(left: Expr<'a>, op: Token<'a>, right: Expr<'a>) -> Self {
        Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Literal { value }
    }
}
