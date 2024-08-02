#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Illegal(Vec<char>),
    EOF,

    // Identifiers + literals
    Ident(Vec<char>),
    Int(Vec<char>),

    // Operators
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    LessThan,
    GreaterThan,

    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Func,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "func" => Token::Func,
        "let" => Token::Let,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Ident(ident.chars().collect()),
    }
}
