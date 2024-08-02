use crate::token::{lookup_ident, Token};

mod test;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    read_pos: usize,
    ch: Option<char>,
    errors: Vec<String>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let input: Vec<char> = input.chars().collect();
        let mut res = Self {
            input,
            pos: 0,
            read_pos: 0,
            ch: None,
            errors: vec![],
        };
        res.read_char();
        res
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input[self.read_pos]);
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.read_pos])
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            Some('=') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            Some('!') => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            Some(';') => Token::Semicolon,
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some(',') => Token::Comma,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,
            Some('/') => Token::Slash,
            Some('<') => Token::LessThan,
            Some('>') => Token::GreaterThan,
            None => Token::EOF,
            Some(a) => {
                let res = if is_letter(a) {
                    let ident = self.read_identifier();
                    lookup_ident(&ident)
                } else if is_digit(a) {
                    Token::Int(self.read_number())
                } else {
                    self.read_char();
                    Token::Illegal(vec![a])
                };
                return res;
            }
        };
        self.read_char();
        tok
    }

    pub fn read_all(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while self.ch.is_some() {
            tokens.push(self.next_token());
        }
        tokens
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_some() && self.ch.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    pub fn read_number(&mut self) -> Vec<char> {
        let mut res = Vec::new();
        while self.ch.is_some() && is_digit(self.ch.unwrap()) {
            res.push(self.ch.unwrap());
            self.read_char();
        }
        res
    }

    pub fn read_identifier(&mut self) -> String {
        let mut res = String::new();
        while self.ch.is_some() && is_letter(self.ch.unwrap()) {
            res.push(self.ch.unwrap());
            self.read_char();
        }
        res
    }
}

fn is_letter(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}
