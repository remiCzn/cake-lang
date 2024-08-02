use precedence::PREFIX;

use crate::{
    ast::{
        self,
        ident::Ident,
        if_else::IfExpression,
        let_stat::Let,
        operators::{lookup_infix, lookup_prefix},
        return_stat::Return,
        Expr, Program, Statement,
    },
    lexer,
    token::{self, Token},
};

mod precedence;
mod test;

#[derive(Debug)]
pub struct Parser {
    lexer: lexer::Lexer,

    current_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn debug(&self) {
        println!(
            "current token: {:?} / peek token: {:?}",
            self.current_token, self.peek_token
        );
    }

    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn expect_peek(&mut self, token: Token) -> bool {
        if std::mem::discriminant(&self.peek_token) != std::mem::discriminant(&token) {
            println!("Error: expected: {:?}, got: {:?}", token, self.peek_token);
            return false;
        }
        self.next_token();
        true
    }

    pub fn parse(&mut self) -> ast::Program {
        let mut statements = Vec::new();
        while Token::EOF != self.current_token.clone() {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }
        Program { statements }
    }

    pub fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.current_token.clone() {
            Token::EOF => None,
            Token::Let => {
                if !self.expect_peek(Token::Ident(vec![])) {
                    return None;
                }

                if let Token::Ident(a) = self.current_token.clone() {
                    let name: String = a.into_iter().collect();

                    if !self.expect_peek(Token::Assign) {
                        return None;
                    }
                    self.next_token();
                    let value = self.parse_expression(precedence::LOWEST);

                    if !self.expect_peek(Token::Semicolon) {
                        return None;
                    }

                    return Some(Statement::Let(Let {
                        ident: Ident { name },
                        expr: value.unwrap(),
                    }));
                }
                return None;
            }
            Token::Return => {
                self.next_token();
                if let Some(expr) = self.parse_expression(precedence::LOWEST) {
                    if !self.expect_peek(Token::Semicolon) {
                        return None;
                    }
                    return Some(Statement::Return(Return { return_expr: expr }));
                }
                None
            }
            Token::LBrace => {
                self.next_token();
                let mut statements = Vec::new();
                while Token::RBrace != self.current_token && Token::EOF != self.current_token {
                    if let Some(statement) = self.parse_statement() {
                        statements.push(statement);
                    }
                    self.next_token();
                }
                return Some(Statement::BlockStatement(statements));
            }
            Token::If => {
                if !self.expect_peek(Token::LParen) {
                    return None;
                }
                self.next_token();
                let condition = self.parse_expression(precedence::LOWEST);

                if !self.expect_peek(Token::RParen) {
                    return None;
                }

                if !self.expect_peek(Token::LBrace) {
                    return None;
                }

                let then = self.parse_statement();

                if let None = then {
                    return None;
                }

                if self.peek_token != Token::Else {
                    return Some(Statement::IfExpression(IfExpression {
                        condition: condition.unwrap(),
                        then: Box::new(then.unwrap()),
                        else_: None,
                    }));
                }
                self.next_token();

                if !self.expect_peek(Token::LBrace) {
                    return None;
                }

                let else_ = self.parse_statement();

                if let None = else_ {
                    return None;
                }

                Some(Statement::IfExpression(IfExpression {
                    condition: condition.unwrap(),
                    then: Box::new(then.unwrap()),
                    else_: Some(Box::new(else_.unwrap())),
                }))
            }
            _ => {
                if let Some(expr) = self.parse_expression(precedence::LOWEST) {
                    if !self.expect_peek(Token::Semicolon) {
                        return None;
                    }
                    Some(Statement::Expression(expr))
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_expression(&mut self, prec: i32) -> Option<Expr> {
        let mut left = self.parse_prefix();

        while self.peek_token != Token::Semicolon
            && prec < precedence::get_precedence(self.peek_token.clone())
        {
            if let Some(left_ex) = left.clone() {
                self.next_token();
                let parsed_infix = self.parse_infix(left_ex.clone());
                if let None = parsed_infix {
                    return Some(left_ex);
                }

                left = parsed_infix;
            } else {
                return left;
            }
        }
        return left;
    }

    pub fn parse_prefix(&mut self) -> Option<Expr> {
        let res = match self.current_token.clone() {
            Token::Ident(a) => {
                let name: String = a.into_iter().collect();
                Some(Expr::Ident(Ident { name }))
            }
            Token::Int(a) => {
                let num: String = a.into_iter().collect();
                Some(Expr::Int(num.parse::<i64>().unwrap()))
            }
            Token::True => Some(Expr::Bool(true)),
            Token::False => Some(Expr::Bool(false)),
            Token::Minus | Token::Plus | Token::Bang => {
                let op = self.current_token.clone();
                self.next_token();
                if let Some(prefix_op) = lookup_prefix(op) {
                    if let Some(expr) = self.parse_expression(PREFIX) {
                        Some(Expr::Prefix(prefix_op, Box::new(expr)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Token::LParen => {
                self.next_token();
                if let Some(expr) = self.parse_expression(precedence::LOWEST) {
                    if !self.expect_peek(Token::RParen) {
                        return None;
                    }
                    return Some(expr);
                }
                None
            }
            Token::Func => {
                self.next_token();
                if self.expect_peek(Token::LParen) {
                    return None;
                }

                let params = if self.peek_token == Token::RParen {
                    self.next_token();
                    Vec::new()
                } else {
                    self.next_token();
                    let mut params = Vec::new();

                    if let Token::Ident(ident) = self.current_token.clone() {
                        params.push(Ident {
                            name: ident.into_iter().collect(),
                        })
                    } else {
                        return None;
                    }

                    while self.peek_token == Token::Comma {
                        self.next_token();
                        self.next_token();
                        if let Token::Ident(ident) = self.current_token.clone() {
                            params.push(Ident {
                                name: ident.into_iter().collect(),
                            })
                        }
                    }

                    if !self.expect_peek(Token::RParen) {
                        return None;
                    }
                    params
                };
                self.next_token();

                let body = self.parse_statement();

                if let Some(Statement::BlockStatement(block)) = body {
                    return Some(Expr::Function(
                        params,
                        Box::new(Statement::BlockStatement(block)),
                    ));
                }
                None
            }
            _ => None,
        };
        res
    }

    pub fn parse_infix(&mut self, left: Expr) -> Option<Expr> {
        match self.current_token {
            Token::Plus
            | Token::Minus
            | Token::Star
            | Token::Slash
            | Token::LessThan
            | Token::GreaterThan
            | Token::Eq
            | Token::NotEq => {
                let prec = precedence::get_precedence(self.current_token.clone());
                let parsed_infix = lookup_infix(self.current_token.clone());

                self.next_token();

                let parsed_right = self.parse_expression(prec);

                if let Some(right) = parsed_right {
                    if let Some(infix_op) = parsed_infix {
                        return Some(Expr::Infix(Box::new(left), infix_op, Box::new(right)));
                    }
                }
                None
            }
            Token::LParen => {
                let mut params = Vec::new();
                if self.peek_token == Token::RParen {
                    self.next_token();
                    return Some(Expr::Call(Box::new(left), params));
                }
                self.next_token();
                let first_arg = self.parse_expression(precedence::LOWEST);
                params.push(first_arg.unwrap());

                while self.peek_token == Token::Comma {
                    self.next_token();
                    self.next_token();
                    params.push(self.parse_expression(precedence::LOWEST).unwrap());
                }

                if !self.expect_peek(Token::RParen) {
                    return None;
                }

                return Some(Expr::Call(Box::new(left), params));
            }
            _ => return None,
        }
    }
}
