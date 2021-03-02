use super::token;
use super::lexer;

pub trait Node {
    fn token_literal(&self) -> String;
    fn string(&self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&self) {}
}

pub unsafe trait Expression: Node {
    fn expression_node(&self) {}
    fn cast_to_expression_statement(&self) -> ExpressionStatement; /* {
        ExpressionStatement {
            token: lexer::Lexer::new(self.token_literal()).next().unwrap(),
            expression: Some(Box::new(self)),
        }
    } */
}
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if (*self).statements.len() > 0 {
            (*self).statements[0].token_literal()
        } else { String::from("") }
    }

    fn string(&self) -> String {
        let mut out = String::new();
        for s in &self.statements {
            out.push_str(s.string().as_str());
        }
        out
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String { self.token.literal.clone() }

    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str({
            let mut token_literal = self.token_literal();
            token_literal.push_str(" ");
            token_literal
        }.as_str());

        out.push_str(self.name.string().as_str());
        out.push_str(" = ");

        if let Some(expression) = &self.value {
            out.push_str(expression.string().as_str());
        }

        out.push_str(";");
        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

pub struct ReturnStatement {
    pub token: token::Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String { return self.token.literal.clone() }

    fn string(&self) -> String {
        if let Some(expression) = &self.expression {
            return expression.string()
        }
        String::new()
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) { }
}

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String {
        if let Some(expression) = &self.expression {
            return expression.string()
        }
        String::new()
    }
}

impl Statement for ExpressionStatement {}

pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String { self.token.literal.clone() }

    fn string(&self) -> String {
        let mut out = String::new();

        for s in self.statements.iter() {
            out.push_str(s.string().as_str());
        }
        out
    }
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String { self.value.clone() }
}

impl Expression for Identifier {}

pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String { self.value.to_string() }
}

impl Expression for Boolean {}

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String { self.token.literal.clone() }
}

impl Expression for IntegerLiteral {}

pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");//)
        out.push_str(self.operator.as_str());
        out.push_str(self.right.string().as_str());
        out.push_str(")");
        out
    }
}

impl Expression for PrefixExpression {}

pub struct InfixExpression {
    pub token: token::Token,
    pub left: Box<dyn Expression>,
    pub operator: String,
    pub right: Box<dyn Expression>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("(");//)
        out.push_str(self.left.string().as_str());
        out.push_str(&(" ".to_owned() + self.operator.as_str() + " "));
        out.push_str(self.right.string().as_str());
        out.push_str(")");
        out
    }
}

impl Expression for InfixExpression {}

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Box<dyn Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String {
        let mut out = String::new();
        out.push_str("if");
        out.push_str(self.condition.string().as_str());
        out.push_str(" ");
        out.push_str(self.consequence.string().as_str());
        if let Some(alternative) = &self.alternative {
            let alternative = alternative.string();
            out.push_str("else ");
            out.push_str(alternative.as_str());
        }
        out
    }
}

impl Expression for IfExpression {}

pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Option<Identifier>>,
    pub blockstatement: Option<BlockStatement>,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String {
        let mut out = String::new();
        let mut params = Vec::new();
        for param in &self.parameters {
            if let Some(param)  = param {
                params.push(param.string());
            }
        }

        out.push_str(self.token_literal().as_str());
        out.push_str("(");//)
        out.push_str(params.join(", ").as_str());
        out.push_str(") ");
        if let Some(body) = &self.blockstatement {
            out.push_str(body.string().as_str());
        }
        out
    }
}

impl Expression for FunctionLiteral {}

pub struct CallExpression {
    pub token: token::Token,
    pub function: Box<dyn Expression>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String { self.token.literal.clone() }
    fn string(&self) -> String {
        let mut out = String::new();
        let mut args: Vec<String> = Vec::new();
        for arg in &self.arguments {
            args.push(arg.string());
        }

        out.push_str(self.function.string().as_str());
        out.push_str("(");//)
        out.push_str(&args.join(", "));
        out.push_str(")");//(

        out
    }
}