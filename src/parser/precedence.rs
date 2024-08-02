use crate::token::Token;

pub const LOWEST: i32 = 0;
pub const EQUALS: i32 = 1;
pub const LESS_GREATER: i32 = 2;
pub const SUM: i32 = 3;
pub const PRODUCT: i32 = 4;
pub const PREFIX: i32 = 5;
pub const CALL: i32 = 6;

pub fn get_precedence(op: Token) -> i32 {
    match op {
        Token::Eq => EQUALS,
        Token::NotEq => EQUALS,
        Token::LessThan => LESS_GREATER,
        Token::GreaterThan => LESS_GREATER,
        Token::Plus => SUM,
        Token::Minus => SUM,
        Token::Star => PRODUCT,
        Token::Slash => PRODUCT,
        Token::LParen => CALL,
        _ => LOWEST,
    }
}
