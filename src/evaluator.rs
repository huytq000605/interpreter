use crate::parser::Program;
use crate::statement::{ExpressionStatement, ExpressionStatement::*, Statement::*};
use crate::token::Token;
use std::collections::HashMap;
use std::fmt::format;

pub struct Evaluator {
    program: Program,
}

#[derive(Debug)]
enum Object {
    Number(f64),
    String(String),
    Null,
}

struct Environment {
    variables: HashMap<String, Object>,
}

impl Evaluator {
    fn new(program: Program) -> Self {
        return Evaluator { program };
    }

    fn eval(self, environment: Environment) {
        for statement in self.program.statements.into_iter() {
            match statement {
                Let(variable_name, value) => {
                    // environment.variables.insert(variable_name, Object)
                }
                Return(return_value) => {}
                Expression(expr_statement) => {}
            }
        }
    }

    fn eval_expression(self, expr: &ExpressionStatement) -> Result<Object, String> {
        match expr {
            Prefix { operator, right } => {
                let mut v = self.eval_expression(right)?;
                v = match operator {
                    Token::Bang => match v {
                        Object::Number(v) => {
                            if v == 0 as f64 {
                                Object::Number(1 as f64)
                            } else {
                                Object::Number(0 as f64)
                            }
                        }
                        _ => {
                            return Err(format!(
                                "Invalid value, operatopr = {:?}, value = {:?}",
                                operator, v
                            ))
                        }
                    },
                    Token::Minus => match v {
                        Object::Number(v) => Object::Number(-v),
                        _ => {
                            return Err(format!(
                                "Invalid value, operatopr = {:?}, value = {:?}",
                                operator, v
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Invalid prefix operator, operatopr = {:?}",
                            operator
                        ))
                    }
                };
                Ok(v)
            }
            Infix {
                left,
                operator,
                right,
            } => return Err("Unimplemented".to_string()),
            If {
                condition,
                outcome,
                alternate,
            } => return Err("Unimplemented".to_string()),
            Fn { args, body } => Err("Unimplemented".to_string()),
            Call { caller, args } => Err("Unimplemented".to_string()),
            Group(expr) => self.eval_expression(expr),
            Identifier(s) => Ok(Object::String(s.to_string())),
            Num(num) => Ok(Object::Number(*num)),
            Bool(b) => {
                if *b {
                    Ok(Object::Number(1 as f64))
                } else {
                    Ok(Object::Number(0 as f64))
                }
            }
        }
    }
}

mod test {
	use super::*;

	#[test]
	fn test_evaluator() {}
}