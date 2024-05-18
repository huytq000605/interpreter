use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub identifier: String,
    pub value: Option<ExpressionStatement>,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    pub value: Option<ExpressionStatement>,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionStatement {
    Prefix(PrefixExpression),
    Infix(InflixExpression),
    Identifier(String),
    Num(f64),
}

#[derive(Debug, PartialEq)]
pub struct PrefixExpression {
    pub right: Box<ExpressionStatement>,
}

#[derive(Debug, PartialEq)]
pub struct InflixExpression {
    pub operator: Token,
    pub left: Box<ExpressionStatement>,
    pub right: Box<ExpressionStatement>,
}
