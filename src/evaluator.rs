use crate::object::{Environment, Object};
use crate::parser::Program;
use crate::statement::{ExpressionStatement, ExpressionStatement::*, Statement::*};
use crate::token::Token;
use std::collections::HashMap;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn eval(&self, program: Program, environment: &mut Environment) -> Object {
        let mut last_v = Object::Null;
        for statement in program.statements.iter() {
            match statement {
                Let(variable_name, value) => {
                    let v = match value {
                        None => Object::Null,
                        Some(expr) => {
                            let v = self.eval_expression(&environment, &expr);
                            match v {
                                Err(e) => panic!("{}", e),
                                Ok(v) => v,
                            }
                        }
                    };
                    last_v = v.clone();
                    environment.variables.insert(variable_name.clone(), v);
                }
                Return(return_value) => {}
                Expression(expr_statement) => {}
            }
        }
        return last_v;
    }

    fn eval_expression(
        &self,
        environment: &Environment,
        expr: &ExpressionStatement,
    ) -> Result<Object, String> {
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
                    _ => return Err(format!("Invalid infix operator {:?}", operator)),
                }
            }
            If {
                condition,
                outcome,
                alternate,
            } => return Err("Unimplemented".to_string()),
            Fn { args, body } => Err("Unimplemented".to_string()),
            Call { caller, args } => Err("Unimplemented".to_string()),
            Group(expr) => self.eval_expression(environment, expr),
            Identifier(s) => match environment.variables.get(s) {
                Some(v) => Ok(v.to_owned()),
                None => Err(format!("Undefined variable {}", s)),
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
    use crate::{evaluator, lexer, parser};

    use super::*;

    #[test]
    fn test_evaluator() {
        struct Testcase<'a> {
            name: &'a str,
            input: String,
            expected: Object,
        }
        let testcases: Vec<Testcase> = vec![Testcase {
            name: "evaluate some add operations",
            input: String::from(
                "let a = 5",
            ),
            expected: Object::Number(5.0),
        }];

        for testcase in testcases.into_iter() {
            let evaluator = Evaluator::new();
            let mut env = Environment::new();
            let lexer = lexer::Lexer::new(&testcase.input);
            let mut parser = parser::Parser::new(lexer);
            let program = match parser.parse_program() {
                Err(e) => {
                    println!("There was error during parsing, err={:?}", e);
                    Program { statements: vec![] }
                }
                Ok(program) => program,
            };
            let v = evaluator.eval(program, &mut env);
            assert_eq!(v, testcase.expected);
        }
    }
}
