use crate::parser::Program;
use crate::statement::{ExpressionStatement, ExpressionStatement::*, Statement::*};
use crate::token::Token;
use std::collections::HashMap;
use std::env::var;
use std::ops;

pub struct Evaluator {
    program: Program,
}

#[derive(Debug, Clone)]
enum Object {
    Number(f64),
    String(String),
    Null,
}

impl ops::Add for Object {
    type Output = Result<Object, String>;

    fn add(self, rhs: Self) -> Result<Object, String> {
        return match self {
            Self::Number(num1) => match rhs {
                Self::Number(num2) => Ok(Self::Number(num1 + num2)),
                Self::String(s2) => Ok(Self::String(format!("{}{}", num1, s2))),
                Self::Null => return Err(format!("Invalid value rhs = NULL"))
            },
            Self::String(s1) => match rhs {
                Self::Number(num2) => Ok(Self::String(format!("{}{}", s1, num2))),
                Self::String(s2) => Ok(Self::String(s1 + &s2)),
                Self::Null => return Err(format!("Invalid value rhs = NULL"))
            },
            Self::Null => return Err(format!("Invalud value lhs = NULL"))
        }
    }
}

impl ops::Sub for Object {
    type Output = Result<Object, String>;

    fn sub(self, rhs: Self) -> Result<Object, String> {
        return match self {
            Self::Number(num1) => match rhs {
                Self::Number(num2) => Ok(Self::Number(num1 - num2)),
                _ => return Err(format!("Invalid value rhs = {:?}", self))
            },
            _ => return Err(format!("Invalud value lhs = {:?}", self))
        }
    }
}

struct Environment {
    variables: HashMap<String, Object>,
}

impl Evaluator {
    fn new(program: Program) -> Self {
        return Evaluator { program };
    }

    fn eval(self, environment: Environment){
        for statement in self.program.statements.into_iter() {
            match statement {
                Let(variable_name, value) => {
                    match value {
                        None => environment.variables.insert(variable_name, Object::Null),
                        Some(expr) => {
                            let v = self.eval_expression(&environment, &expr);
                            match v {
                                Err(e) => panic!("{}", e),
                                Ok(v) => {
                                    environment.variables.insert(variable_name, v);
                                }
                            }
                        }
                    }
                    let mut v = Object::Null;
                    if value
                    // environment.variables.insert(variable_name, Object)
                }
                Return(return_value) => {}
                Expression(expr_statement) => {}
            }
        }
    }

    fn eval_expression(&self, environment: &Environment, expr: &ExpressionStatement) -> Result<Object, String> {
        match expr {
            Prefix { operator, right } => {
                let mut v = self.eval_expression(environment, right)?;
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
            } => {
                let lhs = self.eval_expression(environment, left)?;
                let rhs = self.eval_expression(environment, right)?;
                match *operator {
                    Token::Plus => lhs + rhs,
                    Token::Minus => lhs - rhs,
                    _ => return Err(format!("Invalid infix operator {:?}", operator))
                }
            },
            If {
                condition,
                outcome,
                alternate,
            } => return Err("Unimplemented".to_string()),
            Fn { args, body } => Err("Unimplemented".to_string()),
            Call { caller, args } => Err("Unimplemented".to_string()),
            Group(expr) => self.eval_expression(environment, expr),
            Identifier(s) => {
                match environment.variables.get(s) {
                    Some(v) => Ok(v.to_owned()),
                    None => Err(format!("Undefined variable {}", s)),
                }
            },
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