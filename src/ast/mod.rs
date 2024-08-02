use operators::{InfixOperator, PrefixOperator};

pub mod block;
pub mod ident;
pub mod if_else;
pub mod let_stat;
pub mod operators;
pub mod return_stat;

pub trait Node {
    fn literal(&self) -> String;
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expr {
    Ident(ident::Ident),
    Int(i64),
    Bool(bool),
    Illegal(String),
    Prefix(PrefixOperator, Box<Expr>),
    Infix(Box<Expr>, InfixOperator, Box<Expr>),
    Function(Vec<ident::Ident>, Box<Statement>),
    Call(Box<Expr>, Vec<Expr>),
}

impl Node for Expr {
    fn literal(&self) -> String {
        match self {
            Expr::Ident(ident) => ident.literal(),
            Expr::Illegal(s) => s.clone(),
            Expr::Int(i) => i.to_string(),
            Expr::Prefix(op, expr) => format!("({:?}{})", op, expr.literal()),
            Expr::Infix(left, op, right) => {
                format!("({} {:?} {})", left.literal(), op, right.literal())
            }
            Expr::Bool(x) => x.to_string(),
            Expr::Function(parameters, body) => {
                format!("Func{} {}", parameters.literal(), body.literal())
            }
            Expr::Call(func, params) => {
                let params = params.iter().map(|x| x.literal()).collect::<Vec<String>>();
                format!("{}({})", func.literal(), params.join(","))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Let(let_stat::Let),
    Return(return_stat::Return),
    Expression(Expr),
    Illegal(String),
    BlockStatement(Vec<Statement>),
    IfExpression(if_else::IfExpression),
}

impl Node for Statement {
    fn literal(&self) -> String {
        match self {
            Statement::Let(let_stat) => let_stat.literal(),
            Statement::Return(return_stat) => return_stat.literal(),
            Statement::Expression(expr) => expr.literal(),
            Statement::Illegal(s) => s.clone(),
            Statement::BlockStatement(block) => block.literal(),
            Statement::IfExpression(cond) => cond.literal(),
        }
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn literal(&self) -> String {
        if self.statements.len() == 0 {
            String::from("Program {}")
        } else {
            let mut res = String::from("Program {\n");
            for statement in &self.statements {
                res.push_str("    ");
                res.push_str(&statement.literal());
                res.push_str(";\n");
            }
            res.push_str("}");
            res
        }
    }
}
