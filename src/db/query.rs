use super::parser::ASTNode;





#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Identifier(pub String);

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}

#[derive(Debug)]
pub struct QueryPlan {
    pub projection: Vec<Identifier>,
    pub table: Identifier,
}

pub struct QueryPlanner {}

impl QueryPlanner {
    pub fn new() -> Self {
        QueryPlanner {}
    }

    pub fn plan(&self, ast: &ASTNode) -> QueryPlan {
        match ast {
            ASTNode::SelectStatement { projection, table } => QueryPlan {
                projection: projection.clone(),
                table: table.clone()
            },
            _ => unimplemented!("Only SELECT statements are supported.")
        }
    }
}


