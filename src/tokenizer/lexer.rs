use std::collections::HashMap;

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

pub struct Lexer {
    cur_char: char,
    position: usize,
    read_position: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut ret = Self {
            cur_char: '\0',
            position: 0,
            read_position: 0,
            input: input.chars().collect(),
        };
        ret.read_char();
        return ret;
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.cur_char {
            '\0' => Token::Eof,

            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            },

            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBracket,
            '}' => Token::RBracket,

            '1'..='9' => {
                let mut num = String::from(self.cur_char);
                let mut is_float = false;
                loop {
                    let next_char = self.peek_char();
                    if next_char.is_alphanumeric() || next_char == '.' {
                        if next_char == '.' {
                            is_float = true;
                        }
                        num.push(next_char);
                        self.read_char();
                    } else {
                        break;
                    }
                }

                let token: Token;
                if is_float {
                    match num.parse() {
                        Ok(f) => token = Token::Float(f),
                        Err(e) => {
                            eprintln!("parse float error = {e}");
                            token = Token::Illegal
                        }
                    }
                } else {
                    match num.parse() {
                        Ok(i) => token = Token::Int(i),
                        Err(e) => {
                            eprintln!("parse int error = {e}");
                            token = Token::Illegal
                        }
                    }
                }

                token
            }
            _ => {
                let mut literal = String::from(self.cur_char);
                loop {
                    let next_char = self.peek_char();
                    if next_char.is_ascii_alphabetic() || next_char == '_' {
                        literal.push(next_char);
                        self.read_char();
                    } else {
                        break;
                    }
                }

                Lexer::literal_to_token(&literal)
            }
        };

        self.read_char();
        while self.cur_char == ' ' {
            self.read_char();
        }
        return token;
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.input[self.position];
            self.position += 1;
        }
    }

    fn peek_char(&mut self) -> char {
        if self.position >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.position];
        }
    }

    fn literal_to_token(literal: &str) -> Token {
        match literal {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(literal.to_string())
        }
    }
}
