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
    Int(i128),
    Float(f64),
}

// impl PartialEq<Token> for &Token {
//     fn eq(&self, other: &Token) -> bool {
//         return (*self).eq(other);
//     }
// }