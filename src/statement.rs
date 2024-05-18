#[derive(PartialEq, Debug)]
pub enum Kind {
	LetStatement
}

pub trait Statement {
	fn kind(&self) -> Kind;
}

pub struct LetStatement {
	pub literal: String,
	pub expression: Option<Expression>
}

impl Statement for LetStatement {
	fn kind(&self) -> Kind {
			return Kind::LetStatement
	}
}

pub struct Expression {

}