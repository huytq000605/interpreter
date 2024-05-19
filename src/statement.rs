use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let {
        identifier: String,
        value: Option<ExpressionStatement>,
    },
    Return {
        value: Option<ExpressionStatement>,
    },
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
    Identifier(String),
    If {
        condition: Box<ExpressionStatement>,
        outcome: Vec<Statement>,
        alternate: Vec<Statement>
    },
    Num(f64),
}