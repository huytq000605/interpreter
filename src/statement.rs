use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(String, Option<ExpressionStatement>),
    Return(Option<ExpressionStatement>),
    Expression(ExpressionStatement),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionStatement {
    PrefixExpression {
        operator: Token,
        right: Box<ExpressionStatement>,
    },
    Infix {
        left: Box<ExpressionStatement>,
        operator: Token,
        right: Box<ExpressionStatement>,
    },
    If {
        condition: Box<ExpressionStatement>,
        outcome: Vec<Statement>,
        alternate: Vec<Statement>,
    },
    Fn {
        args: Vec<String>,
        body: Vec<Statement>,
    },
    Identifier(String),
    Num(f64),
    Bool(bool),
}
