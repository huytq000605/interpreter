use crate::{lexer::Lexer, statement::*, token::Token};
use std::default::Default;

type Precedence = i8;
const PRECEDENCE_LOWEST: Precedence = 0;
const PRECEDENCE_EQUAL: Precedence = 1; // ==
const PRECEDENCE_GREATER_LESS: Precedence = 2; // >, >=, <, <=
const PRECEDENCE_SUM: Precedence = 3; // + -
const PRECEDENCE_PRODUCT: Precedence = 4; // * /
const PRECEDENCE_PREFIX: Precedence = 5; // !X, -X
const PRECEDENCE_PARENTHESE: Precedence = 6; // ()
const PRECEDENCE_INDEX: Precedence = 7; // A[i]

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

pub struct Program {
    pub statements: Vec<Statement>,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            cur_token,
            peek_token,
        }
    }

    pub fn next_token(&mut self) {
        std::mem::swap(&mut self.cur_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: vec![],
            errors: vec![],
        };

        while self.cur_token != Token::Eof {
            match self.cur_token {
                Token::Let => match self.parse_let_statement() {
                    Ok(statement) => {
                        program.statements.push(Statement::Let(statement));
                    }
                    Err(err_msg) => {
                        program.errors.push(err_msg);
                        return program;
                    }
                },
                Token::Return => match self.parse_return_statement() {
                    Ok(statement) => {
                        program.statements.push(Statement::Return(statement));
                    }
                    Err(err_msg) => {
                        program.errors.push(err_msg);
                        return program;
                    }
                },
                _ => match self.parse_expression_statement(PRECEDENCE_LOWEST) {
                    Ok(statement) => {
                        program.statements.push(Statement::Expression(statement));
                    }
                    Err(err_msg) => {
                        program.errors.push(err_msg);
                        return program;
                    }
                },
            }

            // Skip through last token from parsed statement
            self.next_token();
            // Skip through semi colon (optional)
            if self.cur_token == Token::Semicolon {
                self.next_token();
            }
        }

        program
    }

    fn cur_precedence(&self) -> Precedence {
        match self.cur_token {
            Token::LBracket => PRECEDENCE_INDEX,
            Token::LParen => PRECEDENCE_PARENTHESE,
            Token::Equal | Token::NotEqual => PRECEDENCE_EQUAL,
            Token::Plus | Token::Minus => PRECEDENCE_SUM,
            Token::Asterisk | Token::Slash => PRECEDENCE_PRODUCT,
            Token::Gt | Token::Gte | Token::Lt | Token::Lte => PRECEDENCE_GREATER_LESS,
            _ => PRECEDENCE_LOWEST,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Token::LBracket => PRECEDENCE_INDEX,
            Token::LParen => PRECEDENCE_PARENTHESE,
            Token::Equal | Token::NotEqual => PRECEDENCE_EQUAL,
            Token::Plus | Token::Minus => PRECEDENCE_SUM,
            Token::Asterisk | Token::Slash => PRECEDENCE_PRODUCT,
            Token::Gt | Token::Gte | Token::Lt | Token::Lte => PRECEDENCE_GREATER_LESS,
            _ => PRECEDENCE_LOWEST,
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, String> {
        // Skip through let token
        self.next_token();
        let literal = match &self.cur_token {
            Token::Ident(literal) => literal.clone(),
            _ => return Err("Invalid let statement".to_string()),
        };
        if self.peek_token != Token::Assign {
            return Ok(LetStatement {
                identifier: literal,
                value: None,
            });
        }

        // Skip through identifier token
        self.next_token();
        // Skip through assign token
        self.next_token();

        let let_statement = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => LetStatement {
                identifier: literal,
                value: Some(statement),
            },
            Err(e) => {
                return Err(e);
            }
        };

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(let_statement)
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, String> {
        // Skip through return token
        self.next_token();
        match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => Ok(ReturnStatement {
                value: Some(statement),
            }),
            Err(e) => Err(e),
        }
    }

    fn parse_expression_statement(
        &mut self,
        precedence: Precedence,
    ) -> Result<ExpressionStatement, String> {
        // Match Prefix Parse
        let mut left = match &self.cur_token {
            Token::Num(num) => ExpressionStatement::Num(*num),
            Token::Ident(literal) => ExpressionStatement::Identifier(literal.clone()),
            _ => {
                return Err(format!(
                    "No prefix parse arm of token = {:?}",
                    self.cur_token
                ))
            }
        };


        // Match Infix Parse
        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            left = match &self.peek_token {
                Token::Plus | Token::Minus | Token::Slash | Token::Asterisk => {
                    // Skip through prefix expression
                    self.next_token();
                    let precedence = self.cur_precedence();
                    let operator = self.cur_token.clone();
                    // Skip through operator token
                    self.next_token();
                    match self.parse_expression_statement(precedence) {
                        Ok(right) => ExpressionStatement::Infix(InflixExpression {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        }),
                        Err(e) => return Err(e),
                    }
                }
                _ => return Ok(left),
            };
        }

        Ok(left)
    }
}

#[cfg(test)]
mod test {

    use crate::statement::{ExpressionStatement, LetStatement, ReturnStatement, Statement};

    use super::*;
    #[test]
    fn test_parser() {
        struct Testcase {
            input: String,
            expected: Vec<Statement>,
        }
        let testcases: Vec<Testcase> = vec![
            Testcase {
                input: String::from("let a"),
                expected: vec![Statement::Let(LetStatement {
                    identifier: "a".to_string(),
                    value: None,
                })],
            },
            Testcase {
                input: String::from(
                    "let a = 6;
                    return 5",
                ),
                expected: vec![
                    Statement::Let(LetStatement {
                        identifier: "a".to_string(),
                        value: Some(ExpressionStatement::Num(6 as f64)),
                    }),
                    Statement::Return(ReturnStatement {
                        value: Some(ExpressionStatement::Num(5 as f64)),
                    }),
                ],
            },
            Testcase {
                input: String::from("let a = 5+6+7"),
                expected: vec![Statement::Let(LetStatement {
                    identifier: "a".to_string(),
                    value: Some(ExpressionStatement::Infix(InflixExpression {
                        left: Box::new(ExpressionStatement::Infix(InflixExpression{
                            left: Box::new(ExpressionStatement::Num(5 as f64)),
                            operator: Token::Plus,
                            right: Box::new(ExpressionStatement::Num(6 as f64)),
                        })),
                        operator: Token::Plus,
                        right: Box::new(ExpressionStatement::Num(7 as f64)),
                    })),
                })],
            },
            Testcase {
                input: String::from("let a = 5+6/7"),
                expected: vec![Statement::Let(LetStatement {
                    identifier: "a".to_string(),
                    value: Some(ExpressionStatement::Infix(InflixExpression {
                        left: Box::new(ExpressionStatement::Num(5 as f64)),
                        operator: Token::Plus,
                        right: Box::new(ExpressionStatement::Infix(InflixExpression{
                            left: Box::new(ExpressionStatement::Num(6 as f64)),
                            operator: Token::Slash,
                            right: Box::new(ExpressionStatement::Num(7 as f64)),
                        })),
                    })),
                })],
            },
        ];
        for testcase in testcases.into_iter() {
            let lexer = Lexer::new(&testcase.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            assert_eq!(program.statements.len(), testcase.expected.len(),);
            for (i, statement) in program.statements.into_iter().enumerate() {
                assert_eq!(statement, testcase.expected[i]);
            }
        }
    }
}
