enum Token {
	Eof,
	Ident(String),
	Plus,
	Minus,
	Product,
	Divisor,
}

struct Lexer {
	cur_char: char,
	position: usize,
	read_position: usize,
	input: Vec<char>,
}

impl Lexer {
	pub fn new(input: String) -> Self {
		Self{
			cur_char: '0',
			position: 0,
			read_position: 0,
			input: input.chars().collect(),
		}
	}

	pub fn next_token(&self) -> Token {
		match self.cur_char {
			'+' => Token::Plus,
			_ => {
				let literal = self.read_char();
				return Token::Ident(literal);
			}
		}
	}

	fn peek_token() -> Token {
		Token::Eof
	}

	fn read_char(&self) -> String {
		return "".to_string()
	}
}