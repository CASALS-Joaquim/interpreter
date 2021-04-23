#[allow(dead_code)]

use std::collections::HashMap;
use std::cmp::PartialEq;
use core_stable::token;

pub type Identifier = String;
pub type BlockStatement = Vec<Statement>;
pub type Parameters = Vec<Identifier>;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Bang,
    Personnalised(String),
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum InfixOperator {
    Plus,
    Minus,
    Eq,
    NotEq,
    GT,
    LT,
    GTE,
    LTE,
    Personalised(String),
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum SuffixOperator {
    Bang,
    Personnalised(String),
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Operator {
    Prefix(PrefixOperator),
    Infix(InfixOperator),
    Suffix(SuffixOperator),
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Statement {
    ///Expression
    Expr(Expression),

    /// Keywords
    Let{ name: Identifier, value: Expression },
    Return(Expression),
}

impl From<Expression> for Statement {
    fn from(expr: Expression) -> Statement {
        Statement::Expr(expr)
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Expression {
    /// Identifiers + literals
    Ident(Identifier),
    Int(isize),
    Float(f64),
    String(String),
    Boolean(bool),
    Function{ params: Parameters, body: Box<Expression> },

    /// Expressions
    PrefixExpression{ operator: Operator, right: Box<Expression> },
    InfixExpression{ left: Box<Expression>, op: Operator, right:Box<Expression> },
    PostfixExpression{ right: Box<Expression>, op: Operator },
    IfExpression{ condition: Box<Expression>, consequence: Box<Statement>, alternative: Box<Statement> },
    CallExpression{ lambda: Box<Expression>, parameters: Vec<Expression> },
    BlockExpression(BlockStatement),
    Unit
}

#[allow(dead_code)]
pub struct Program {
    pub global: BlockStatement,
}

impl From<BlockStatement> for Program {
    fn from(global: BlockStatement) -> Program {
        Program {
            global: global
        }
    }
}