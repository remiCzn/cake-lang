use rustyline::{error::ReadlineError, DefaultEditor};

use crate::{ast::Node, lexer, parser};

const PROMPT: &str = "cake-repl > ";

pub fn repl() {
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        let readline = rl.readline("cake-repl > ");
        match readline {
            Ok(line) => {
                let lex = lexer::Lexer::new(&line);
                let mut parser = parser::Parser::new(lex);
                let res = parser.parse();
                println!("{}", res.literal());
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
