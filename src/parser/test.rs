#[cfg(test)]
mod tests {
    use crate::{ast::Node, lexer, parser::Parser};

    #[test]
    fn test_parse_expression() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
                1 + 2 * 3;\
                -a+b;\
                 !-a;\
                 a+ b+ c;\
                 a+ b- c;\
                 a * b * c;\
                 a * b / c;\
                 a+b/c;\
                 a+b*c+d/e-f;\
                 5>4 == 3 < 4;\
                 5<4 != 3>4;\
                 3+4*5 == 3*1+4*5;\
                 ",
        ));
        let prog = parser.parse();

        assert_eq!(prog.statements[0].literal(), "(1 + (2 * 3))");
        assert_eq!(prog.statements[1].literal(), "((-a) + b)");
        assert_eq!(prog.statements[2].literal(), "(!(-a))");
        assert_eq!(prog.statements[3].literal(), "((a + b) + c)");
        assert_eq!(prog.statements[4].literal(), "((a + b) - c)");
        assert_eq!(prog.statements[5].literal(), "((a * b) * c)");
        assert_eq!(prog.statements[6].literal(), "((a * b) / c)");
        assert_eq!(prog.statements[7].literal(), "(a + (b / c))");
        assert_eq!(
            prog.statements[8].literal(),
            "(((a + (b * c)) + (d / e)) - f)"
        );
        assert_eq!(prog.statements[9].literal(), "((5 > 4) == (3 < 4))");
        assert_eq!(prog.statements[10].literal(), "((5 < 4) != (3 > 4))");
        assert_eq!(
            prog.statements[11].literal(),
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"
        );
    }

    #[test]
    fn test_bool_expression() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
                true;\
                false;\
                3>5  == false;\
                3<5  == true;\
                ",
        ));

        let prog = parser.parse();
        println!("{}", prog.statements[0].literal());
        assert_eq!(prog.statements[0].literal(), "true");
        assert_eq!(prog.statements[1].literal(), "false");
        assert_eq!(prog.statements[2].literal(), "((3 > 5) == false)");
        assert_eq!(prog.statements[3].literal(), "((3 < 5) == true)");
    }

    #[test]
    fn test_parse_return() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
                return 1;\
                return 1 + 2;\
                return 1 + 2 * 3;\
                return true == false;\
                return 3> a == true;\
                ",
        ));

        let prog = parser.parse();
        println!("{}", prog.statements[0].literal());
        assert_eq!(prog.statements[0].literal(), "Return(1)");
        assert_eq!(prog.statements[1].literal(), "Return((1 + 2))");
        assert_eq!(prog.statements[2].literal(), "Return((1 + (2 * 3)))");
        assert_eq!(prog.statements[3].literal(), "Return((true == false))");
        assert_eq!(prog.statements[4].literal(), "Return(((3 > a) == true))");
    }

    #[test]
    fn test_parse_let() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
                let a = 1;\
                let b = a + 2;\
                let c = a + b * 3;\
                let d = true == false;\
                let e = 3> a == true;\
                ",
        ));

        let prog = parser.parse();
        println!("{}", prog.statements[0].literal());
        assert_eq!(prog.statements[0].literal(), "Let(a, 1)");
        assert_eq!(prog.statements[1].literal(), "Let(b, (a + 2))");
        assert_eq!(prog.statements[2].literal(), "Let(c, (a + (b * 3)))");
        assert_eq!(prog.statements[3].literal(), "Let(d, (true == false))");
        assert_eq!(prog.statements[4].literal(), "Let(e, ((3 > a) == true))");
    }

    #[test]
    fn test_parse_parentheses() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
                (1 + 2) * 3;\
                (1 + 2) / 3;\
                (1 + 2) * 3 + 4;\
                ",
        ));

        let prog = parser.parse();
        println!("{}", prog.statements[0].literal());
        assert_eq!(prog.statements[0].literal(), "((1 + 2) * 3)");
        assert_eq!(prog.statements[1].literal(), "((1 + 2) / 3)");
        assert_eq!(prog.statements[2].literal(), "(((1 + 2) * 3) + 4)");
    }

    #[test]
    fn test_parse_if() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
                if (1 + 2 > 3) {\
                    let a = 1;\
                    let x = a + 3;\
                    1+2;\
                };\
                if (a == 2) {\
                    let a = 3+b;\
                    let b = 5;\
                    return true;\
                } else {\
                    return false;\
                 };\
                if (true) {
                    let a = 1;
                    a * b;
                } else {
                    let a = 1;
                    if(a * b == 2) {
                        return false;
                    };
                };
            ",
        ));

        let prog = parser.parse();
        println!("{:?}", prog.statements);
        assert_eq!(
            prog.statements[0].literal(),
            "If(((1 + 2) > 3), { Let(a, 1); Let(x, (a + 3)); (1 + 2); })",
            "IF: test 1"
        );
        assert_eq!(
            prog.statements[1].literal(),
            "If((a == 2), { Let(a, (3 + b)); Let(b, 5); Return(true); }, { Return(false); })",
            "IF: test 2"
        );
        assert_eq!(
            prog.statements[2].literal(),
            "If(true, { Let(a, 1); (a * b); }, { Let(a, 1); If(((a * b) == 2), { Return(false); }); })",
            "IF: test 3"
        )
    }

    #[test]
    fn test_parse_function() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
            let f = func(x,y) {\
                x+y;\
            }\
            let g = func(x,y,z) {\
                x+y+z;\
            }
            let h = func() {
                return a;
            }

        ",
        ));

        let prog = parser.parse();
        assert_eq!(
            prog.statements[0].literal(),
            "Let(f, Func(x,y) { (x + y); })"
        );

        assert_eq!(
            prog.statements[1].literal(),
            "Let(g, Func(x,y,z) { ((x + y) + z); })"
        );

        assert_eq!(
            prog.statements[2].literal(),
            "Let(h, Func() { Return(a); })"
        );
    }

    #[test]
    fn test_parse_call() {
        let mut parser = Parser::new(lexer::Lexer::new(
            "\
            add(1, 2+3, 4*5);\
            add(1,2, add(3,4,5));\
            a + add(b*c) + d;\
            ",
        ));

        let prog = parser.parse();
        assert_eq!(prog.statements[0].literal(), "add(1,(2 + 3),(4 * 5))");
        assert_eq!(prog.statements[1].literal(), "add(1,2,add(3,4,5))");
        assert_eq!(prog.statements[2].literal(), "((a + add((b * c))) + d)");
    }
}
