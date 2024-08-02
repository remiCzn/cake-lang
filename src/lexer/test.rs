#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::Token};

    #[test]
    fn test_read_simple_symbol() {
        let mut lexer = Lexer::new("(){}+=;");
        let res = lexer.read_all();
        assert_eq!(
            res,
            vec![
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::RBrace,
                Token::Plus,
                Token::Assign,
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_ident() {
        let mut lexer = Lexer::new("let x = 56;");
        let res = lexer.read_all();
        assert_eq!(
            res,
            vec![
                Token::Let,
                Token::Ident(vec!['x']),
                Token::Assign,
                Token::Int(vec!['5', '6']),
                Token::Semicolon,
            ]
        );
    }

    #[test]
    fn test_function() {
        let mut lexer = Lexer::new(
            r#"func () {
                        let x = 5;
                        let y = 6;
                        if(x > 3) {
                         return true;
                        } else {
                          return x == y;
                        }
                    }"#,
        );
        let res = lexer.read_all();
        assert_eq!(
            res,
            vec![
                Token::Func,
                Token::LParen,
                Token::RParen,
                Token::LBrace,
                Token::Let,
                Token::Ident(vec!['x']),
                Token::Assign,
                Token::Int(vec!['5']),
                Token::Semicolon,
                Token::Let,
                Token::Ident(vec!['y']),
                Token::Assign,
                Token::Int(vec!['6']),
                Token::Semicolon,
                Token::If,
                Token::LParen,
                Token::Ident(vec!['x']),
                Token::GreaterThan,
                Token::Int(vec!['3']),
                Token::RParen,
                Token::LBrace,
                Token::Return,
                Token::True,
                Token::Semicolon,
                Token::RBrace,
                Token::Else,
                Token::LBrace,
                Token::Return,
                Token::Ident(vec!['x']),
                Token::Eq,
                Token::Ident(vec!['y']),
                Token::Semicolon,
                Token::RBrace,
                Token::RBrace
            ]
        );
    }
}
