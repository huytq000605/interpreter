use crate::token::Token;
pub struct Lexer {
    cur_char: char,
    position: usize,
    input: Vec<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut ret = Self {
            cur_char: '\0',
            position: 0,
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
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Gte
                } else {
                    Token::Gt
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Lte
                } else {
                    Token::Lt
                }
            }

            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LCurlyBracket,
            '}' => Token::RCurlyBracket,

            '0'..='9' => {
                let mut num = String::from(self.cur_char);
                loop {
                    let next_char = self.peek_char();
                    if next_char.is_alphanumeric() || next_char == '.' {
                        num.push(next_char);
                        self.read_char();
                    } else {
                        break;
                    }
                }

                let token: Token;
                match num.parse() {
                    Ok(f) => token = Token::Num(f),
                    Err(e) => {
                        eprintln!("parse float error = {e}");
                        token = Token::Illegal
                    }
                }

                token
            }
            'a'..='z' | 'A'..='Z' => {
                let mut literal = String::from(self.cur_char);
                loop {
                    let next_char = self.peek_char();
                    if next_char.is_ascii_alphabetic()
                        || next_char.is_alphanumeric()
                        || next_char == '_'
                    {
                        literal.push(next_char);
                        self.read_char();
                    } else {
                        break;
                    }
                }

                Lexer::literal_to_token(&literal)
            }
            _ => Token::Illegal,
        };

        self.read_char();
        while self.cur_char == ' '
            || self.cur_char == '\t'
            || self.cur_char == '\r'
            || self.cur_char == '\n'
        {
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
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(literal.to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lexer() {
        struct Testcase {
            input: String,
            expected: Vec<Token>,
        }
        let testcases = vec![
            Testcase {
                input: String::from("+-*/==="),
                expected: vec![
                    Token::Plus,
                    Token::Minus,
                    Token::Asterisk,
                    Token::Slash,
                    Token::Equal,
                    Token::Assign,
                ],
            },
            Testcase {
                input: "SON   TUNG".to_string(),
                expected: vec![
                    Token::Ident(String::from("SON")),
                    Token::Ident(String::from("TUNG")),
                ],
            },
            Testcase {
                input: "12345".to_string(),
                expected: vec![Token::Num(12345 as f64)],
            },
            Testcase {
                input: "12345.456".to_string(),
                expected: vec![Token::Num(12345.456)],
            },
            Testcase {
                input: "let x = 5".to_string(),
                expected: vec![
                    Token::Let,
                    Token::Ident("x".to_string()),
                    Token::Assign,
                    Token::Num(5 as f64),
                ],
            },
            Testcase {
                input: "let a = 5+6".to_string(),
                expected: vec![
                    Token::Let,
                    Token::Ident("a".to_string()),
                    Token::Assign,
                    Token::Num(5 as f64),
                    Token::Plus,
                    Token::Num(6 as f64),
                ],
            },
            Testcase {
                input: "if a == 5".to_string(),
                expected: vec![
                    Token::If,
                    Token::Ident("a".to_string()),
                    Token::Equal,
                    Token::Num(5 as f64),
                ],
            },
        ];

        for testcase in testcases.into_iter() {
            let mut lexer = Lexer::new(&testcase.input);
            let mut i = 0;
            loop {
                let token = lexer.next_token();
                if token == Token::Eof {
                    break;
                }
                assert_eq!(token, testcase.expected[i]);
                i += 1;
            }
            assert_eq!(i, testcase.expected.len())
        }
    }
}
