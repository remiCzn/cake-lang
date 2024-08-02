use super::{ident::Ident, Expr, Node};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Let {
    pub ident: Ident,
    pub expr: Expr,
}

impl Node for Let {
    fn literal(&self) -> String {
        format!("Let({}, {})", self.ident.literal(), self.expr.literal())
    }
}
