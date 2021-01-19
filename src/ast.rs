trait Node {
    pub fn token_literal(&self) -> String;
    pub fn string() -> String;
}

trait Statement {
    Node
    pub fn statement_node();
}

trait Expression {
    Node
    pub fn expression_node();
}

struct Program {
    statements Vec<Statement>
}

impl Node for Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements[0].token_literal()
        } else { "" }
    }

    pub fn string(&self) -> String {
        let mut out = String::new();
    }
}