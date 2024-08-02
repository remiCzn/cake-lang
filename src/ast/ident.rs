use super::Node;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ident {
    pub name: String,
}

impl Node for Ident {
    fn literal(&self) -> String {
        self.name.clone()
    }
}
