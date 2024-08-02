use std::fmt::Debug;

use crate::token::Token;

#[derive(PartialEq, Eq, Clone)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Bang,
}

impl Debug for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrefixOperator::Plus => write!(f, "+"),
            PrefixOperator::Minus => write!(f, "-"),
            PrefixOperator::Bang => write!(f, "!"),
        }
    }
}

pub fn lookup_prefix(op: Token) -> Option<PrefixOperator> {
    match op {
        Token::Plus => Some(PrefixOperator::Plus),
        Token::Minus => Some(PrefixOperator::Minus),
        Token::Bang => Some(PrefixOperator::Bang),
        _ => None,
    }
}

#[derive(PartialEq, Eq, Clone)]
pub enum InfixOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    NotEq,
    LessThan,
    GreaterThan,
}

impl Debug for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InfixOperator::Plus => write!(f, "+"),
            InfixOperator::Minus => write!(f, "-"),
            InfixOperator::Star => write!(f, "*"),
            InfixOperator::Slash => write!(f, "/"),
            InfixOperator::Eq => write!(f, "=="),
            InfixOperator::NotEq => write!(f, "!="),
            InfixOperator::LessThan => write!(f, "<"),
            InfixOperator::GreaterThan => write!(f, ">"),
        }
    }
}

pub fn lookup_infix(op: Token) -> Option<InfixOperator> {
    match op {
        Token::Eq => Some(InfixOperator::Eq),
        Token::NotEq => Some(InfixOperator::NotEq),
        Token::LessThan => Some(InfixOperator::LessThan),
        Token::GreaterThan => Some(InfixOperator::GreaterThan),
        Token::Plus => Some(InfixOperator::Plus),
        Token::Minus => Some(InfixOperator::Minus),
        Token::Star => Some(InfixOperator::Star),
        Token::Slash => Some(InfixOperator::Slash),
        _ => None,
    }
}
