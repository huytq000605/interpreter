#[derive(PartialEq, Debug)]
pub enum Token {
    Eof,
    Ident(String),
    Plus,
    Minus,
    Product,
    Division,
    Semicolon,
    If,
    Else,
    Return,
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
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Product,
            '/' => Token::Division,
            ';' => Token::Semicolon,
            '\0' => Token::Eof,
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

                Token::Ident(literal)
            }
        };

        self.read_char();
        while self.cur_char == ' ' {
            self.read_char();
        }
        return token;
    }

    fn peek_token() -> Token {
        Token::Eof
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
}
