#[derive(PartialEq, Debug)]
pub enum Token {
    Eof,
    Illegal,

    Plus,
    Minus,
    Asterisk,
    Slash,
    Assign,
    Equal,
    NotEqual,
    Bang,
    Gt,
    Lt,
    Gte,
    Lte,

    Comma,
    Semicolon,
    LParen,
    RParen,
    LBracket,
    RBracket,

    If,
    Else,
    Return,
    Let,
    Fn,

    Ident(String),
    Num(f64),
}
