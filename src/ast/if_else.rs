use super::{Expr, Node, Statement};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IfExpression {
    pub condition: Expr,
    pub then: Box<Statement>,
    pub else_: Option<Box<Statement>>,
}

impl Node for IfExpression {
    fn literal(&self) -> String {
        if let Some(else_) = &self.else_ {
            format!(
                "If({}, {}, {})",
                self.condition.literal(),
                self.then.literal(),
                else_.literal()
            )
        } else {
            format!("If({}, {})", self.condition.literal(), self.then.literal(),)
        }
    }
}
