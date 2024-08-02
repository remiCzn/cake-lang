use super::{ident::Ident, Node, Statement};

impl Node for Vec<Statement> {
    fn literal(&self) -> String {
        let mut res = String::from("{ ");
        for statement in self {
            res.push_str(&statement.literal());
            res.push_str("; ");
        }
        res.push_str("}");
        res
    }
}

impl Node for Vec<Ident> {
    fn literal(&self) -> String {
        let mut res = String::from("(");

        let names = self.iter().map(|x| x.name.clone()).collect::<Vec<String>>();
        res.push_str(names.join(",").as_str());

        res.push_str(")");
        res
    }
}
