use super::{Expr, Node};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Return {
    pub return_expr: Expr,
}

impl Node for Return {
    fn literal(&self) -> String {
        format!("Return({})", self.return_expr.literal())
    }
}
