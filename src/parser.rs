use crate::{lexer::Lexer, token::Token, statement};
pub struct Parser {
	lexer: Lexer,
	cur_token: Token,
	peek_token: Token,
}


pub struct Program {
	pub statements: Vec<Box<dyn statement::Statement>>,
	pub errors: Vec<String>
}

impl Parser {
	pub fn new(mut lexer: Lexer) -> Self {
		let cur_token = lexer.next_token();
		let peek_token = lexer.next_token();
		Parser{
			lexer: lexer,
			cur_token: cur_token,
			peek_token: peek_token,
		}
	}

	pub fn next_token(&mut self) {
		std::mem::swap(&mut self.cur_token, &mut self.peek_token);
		self.peek_token = self.lexer.next_token();
	}

	pub fn parse_program(&mut self) -> Program{
		let mut statements = vec![];
		let mut errors = vec![];
		while self.cur_token != Token::Eof {
			match self.cur_token {
				Token::Let => {
					match self.parse_let_statement() {
						Ok(statement) => {
							statements.push(Box::new(statement) as Box<dyn statement::Statement>);
						},
						Err(err_msg) => {
							errors.push(err_msg)
						} 
					}
				}
				_ => {}
			}

			// Skip through last token from parsed statement
			self.next_token();
			// Skip through semi colon (optional)
			if self.cur_token == Token::Semicolon {
				self.next_token();
			}
		}

		Program{
			statements: statements,
			errors: errors,
		}
	}

	fn parse_let_statement(&mut self) -> Result<statement::LetStatement, String>{	
		// Skip through let token
		self.next_token();
		let literal = match &self.cur_token {
			Token::Ident(literal) => literal.clone(),
			_ => {
				return Err("Invalid let statement".to_string())
			}
		};
		if self.peek_token != Token::Assign {
			return Ok(statement::LetStatement{
				literal: literal,
				expression: None
			})
		}

		// Skip through identifier token
		self.next_token();
		// Skip through assign token
		self.next_token();

		// TODO: Parse expression
		let expression = statement::Expression{};
		Ok(statement::LetStatement{
			literal: literal,
			expression: Some(expression)
		})
	}

}