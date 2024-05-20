use crate::{lexer::Lexer, statement::*, token::Token};

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
            match self.parse_statement() {
                Ok(statement) => program.statements.push(statement),
                Err(e) => {
                    panic!("e = {:?}", e.clone());
                    program.errors.push(e);
                }
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

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.cur_token {
            Token::Let => match self.parse_let_statement() {
                Ok(statement) => Ok(statement),
                Err(e) => Err(e),
            },
            Token::Return => match self.parse_return_statement() {
                Ok(statement) => Ok(statement),
                Err(e) => Err(e),
            },
            _ => match self.parse_expression_statement(PRECEDENCE_LOWEST) {
                Ok(statement) => Ok(Statement::Expression(statement)),
                Err(e) => Err(e),
            },
        }
    }

    fn cur_precedence(&self) -> Precedence {
        match self.cur_token {
            Token::LSquareBracket => PRECEDENCE_INDEX,
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
            Token::LSquareBracket => PRECEDENCE_INDEX,
            Token::LParen => PRECEDENCE_PARENTHESE,
            Token::Equal | Token::NotEqual => PRECEDENCE_EQUAL,
            Token::Plus | Token::Minus => PRECEDENCE_SUM,
            Token::Asterisk | Token::Slash => PRECEDENCE_PRODUCT,
            Token::Gt | Token::Gte | Token::Lt | Token::Lte => PRECEDENCE_GREATER_LESS,
            _ => PRECEDENCE_LOWEST,
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        // Skip through let token
        self.next_token();
        let literal = match &self.cur_token {
            Token::Ident(literal) => literal.clone(),
            _ => {
                return Err(format!(
                    "Expected Token::Ident got {:?}",
                    self.cur_token.clone()
                ))
            }
        };
        if self.peek_token != Token::Assign {
            return Ok(Statement::Let(literal, None));
        }

        // Skip through identifier token
        self.next_token();
        // Skip through assign token
        self.next_token();

        let let_statement = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => Statement::Let(literal, Some(statement)),
            Err(e) => {
                return Err(e);
            }
        };

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Ok(let_statement)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        // Skip through return token
        self.next_token();
        match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => Ok(Statement::Return(Some(statement))),
            Err(e) => Err(e),
        }
    }

    fn parse_expression_statement(
        &mut self,
        precedence: Precedence,
    ) -> Result<ExpressionStatement, String> {
        // Match Prefix Parse
        let mut left = match &self.cur_token {
            Token::Bang | Token::Minus => {
                let operator = self.cur_token.clone();
                // Skip through operator token
                self.next_token();

                let right = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
                    Ok(statement) => statement,
                    Err(e) => return Err(e),
                };
                ExpressionStatement::Prefix {
                    operator,
                    right: Box::new(right),
                }
            }
            Token::Num(num) => ExpressionStatement::Num(*num),
            Token::Ident(literal) => ExpressionStatement::Identifier(literal.clone()),
            Token::True => ExpressionStatement::Bool(true),
            Token::False => ExpressionStatement::Bool(false),
            Token::If => match self.parse_if_expression() {
                Ok(statement) => statement,
                Err(e) => return Err(e),
            },
            Token::Fn => match self.parse_fn_expression() {
                Ok(statement) => statement,
                Err(e) => return Err(e),
            },
            _ => {
                return Err(format!(
                    "No Prefix Parse arm for token = {:?}",
                    self.cur_token
                ))
            }
        };

        // Match Infix Parse
        while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
            left = match &self.peek_token {
                Token::Plus
                | Token::Minus
                | Token::Slash
                | Token::Asterisk
                | Token::Equal
                | Token::NotEqual
                | Token::Gt
                | Token::Gte
                | Token::Lt
                | Token::Lte => {
                    // Skip through prefix expression
                    self.next_token();
                    let precedence = self.cur_precedence();
                    let operator = self.cur_token.clone();
                    // Skip through operator token
                    self.next_token();
                    match self.parse_expression_statement(precedence) {
                        Ok(right) => ExpressionStatement::Infix {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        },
                        Err(e) => return Err(e),
                    }
                }
                Token::LParen => {
                    // Skip through prefix expression
                    self.next_token();
                    // Skip through Token::LParen
                    self.next_token();

                    let mut args = vec![];
                    while self.cur_token != Token::RParen {
                        match self.parse_expression_statement(PRECEDENCE_LOWEST) {
                            Ok(e) => args.push(e),
                            Err(e) => return Err(e),
                        };
                        // Skip through expression
                        self.next_token();
                        match self.cur_token {
                            Token::RParen => break,
                            Token::Comma => {
                                self.next_token();
                            }
                            _ => {
                                return Err(format!(
                                    "Expected Token::RParen or Token::Comma, got={:?}",
                                    self.cur_token
                                ))
                            }
                        }
                    }

                    // Skip through Token::RParen
                    self.next_token();

                    ExpressionStatement::Call {
                        Caller: Box::new(left),
                        Args: args,
                    }
                }
                _ => return Ok(left),
            };
        }

        Ok(left)
    }

    fn parse_if_expression(&mut self) -> Result<ExpressionStatement, String> {
        // Skip through IF token
        self.next_token();

        let mut has_lparen = false;
        if self.cur_token == Token::LParen {
            has_lparen = true;
            self.next_token();
        }
        let condition = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => statement,
            Err(e) => return Err(e),
        };
        // Skip through expression
        self.next_token();

        if has_lparen {
            if self.cur_token != Token::RParen {
                return Err(format!("Expected RParen, got = {:?}", self.peek_token));
            }
            // Skip through RParen
            self.next_token()
        }

        if self.cur_token != Token::LCurlyBracket {
            return Err(format!("Expected LBracket, got = {:?}", self.cur_token));
        }

        // Skip through LBracket Token
        self.next_token();

        // Start parsing outcome until facing RBracket
        let mut outcome = vec![];
        while self.cur_token != Token::RCurlyBracket {
            let statement = match self.parse_statement() {
                Ok(statement) => statement,
                Err(e) => return Err(e),
            };
            outcome.push(statement);
            // Skip through expression
            self.next_token();
        }

        if self.peek_token != Token::Else {
            return Ok(ExpressionStatement::If {
                condition: Box::new(condition),
                outcome,
                alternate: vec![],
            });
        }

        // Skip through RBracket Token
        self.next_token();

        // Skip through Else Token
        self.next_token();

        match self.cur_token {
            Token::If => match self.parse_if_expression() {
                Ok(statement) => Ok(ExpressionStatement::If {
                    condition: Box::new(condition),
                    outcome,
                    alternate: vec![Statement::Expression(statement)],
                }),
                Err(e) => return Err(e),
            },
            Token::LCurlyBracket => {
                // Skip through LBracket Token
                self.next_token();

                // Start parsing outcome until facing RBracket
                let mut alternate = vec![];
                while self.cur_token != Token::RCurlyBracket {
                    let statement = match self.parse_statement() {
                        Ok(statement) => statement,
                        Err(e) => return Err(e),
                    };
                    alternate.push(statement);
                    // Skip through expression
                    self.next_token();
                }

                Ok(ExpressionStatement::If {
                    condition: Box::new(condition),
                    outcome,
                    alternate,
                })
            }
            _ => Err(format!(
                "Expected Token::LBracket or Token::If, got = {:?}",
                self.cur_token
            )),
        }
    }

    fn parse_fn_expression(&mut self) -> Result<ExpressionStatement, String> {
        // Skip through fn token
        self.next_token();
        if self.cur_token != Token::LParen {
            return Err(format!("Expected Token::LParen, got={:?}", self.cur_token));
        }
        // Skip through LParen token
        self.next_token();
        let args = match self.parse_fn_args() {
            Ok(args) => args,
            Err(e) => return Err(e),
        };
        if self.cur_token != Token::LCurlyBracket {
            return Err(format!(
                "Expected Token::LCurlyBracket, got={:?}",
                self.cur_token
            ));
        }
        // Skip through LCurlyBracket token
        self.next_token();
        let mut body = vec![];
        while self.cur_token != Token::RCurlyBracket {
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(e) => return Err(e),
            }
            // Skip through statement
            self.next_token()
        }

        return Ok(ExpressionStatement::Fn { args, body });
    }

    fn parse_fn_args(&mut self) -> Result<Vec<String>, String> {
        let mut args = vec![];

        if self.cur_token != Token::RParen {
            match &self.cur_token {
                Token::Ident(arg) => args.push(arg.clone()),
                _ => return Err(format!("Expected Token::Ident, got={:?}", self.cur_token)),
            }
            // Skip through Ident token
            self.next_token();

            while self.cur_token != Token::RParen {
                if self.cur_token != Token::Comma {
                    return Err(format!("Expected Token::Comma, got={:?}", self.cur_token));
                }
                // Skip through Comma token
                self.next_token();

                match &self.cur_token {
                    Token::Ident(arg) => args.push(arg.clone()),
                    _ => return Err(format!("Expected Token::Ident, got={:?}", self.cur_token)),
                }
                // Skip through Ident token
                self.next_token();
            }
        }

        // Skip through RParen token
        self.next_token();

        Ok(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parser() {
        struct Testcase<'a> {
            name: &'a str,
            input: String,
            expected: Vec<Statement>,
        }
        let testcases: Vec<Testcase> = vec![
            Testcase {
                name: "simple let",
                input: String::from("let a"),
                expected: vec![Statement::Let("a".to_string(), None)],
            },
            Testcase {
                name: "let and return",
                input: String::from(
                    "let a = 6;
                    return 5",
                ),
                expected: vec![
                    Statement::Let("a".to_string(), Some(ExpressionStatement::Num(6 as f64))),
                    Statement::Return(Some(ExpressionStatement::Num(5 as f64))),
                ],
            },
            Testcase {
                name: "let and expression parsing",
                input: String::from("let a = 5+6+7"),
                expected: vec![Statement::Let(
                    "a".to_string(),
                    Some(ExpressionStatement::Infix {
                        left: Box::new(ExpressionStatement::Infix {
                            left: Box::new(ExpressionStatement::Num(5 as f64)),
                            operator: Token::Plus,
                            right: Box::new(ExpressionStatement::Num(6 as f64)),
                        }),
                        operator: Token::Plus,
                        right: Box::new(ExpressionStatement::Num(7 as f64)),
                    }),
                )],
            },
            Testcase {
                name: "let and expression with different precedence check",
                input: String::from("let a = 5 + 6 / 7"),
                expected: vec![Statement::Let(
                    "a".to_string(),
                    Some(ExpressionStatement::Infix {
                        left: Box::new(ExpressionStatement::Num(5 as f64)),
                        operator: Token::Plus,
                        right: Box::new(ExpressionStatement::Infix {
                            left: Box::new(ExpressionStatement::Num(6 as f64)),
                            operator: Token::Slash,
                            right: Box::new(ExpressionStatement::Num(7 as f64)),
                        }),
                    }),
                )],
            },
            Testcase {
                name: "if expression",
                input: String::from(
                    "if a == 5 {
                        let b = 10;
                    } else if c >= 2 {
                        !3
                    } else {
                        gg
                    }
                    ",
                ),
                expected: vec![Statement::Expression(ExpressionStatement::If {
                    condition: Box::new(ExpressionStatement::Infix {
                        left: Box::new(ExpressionStatement::Identifier("a".to_string())),
                        operator: Token::Equal,
                        right: Box::new(ExpressionStatement::Num(5 as f64)),
                    }),
                    outcome: vec![Statement::Let(
                        "b".to_string(),
                        Some(ExpressionStatement::Num(10 as f64)),
                    )],
                    alternate: vec![Statement::Expression(ExpressionStatement::If {
                        condition: Box::new(ExpressionStatement::Infix {
                            left: Box::new(ExpressionStatement::Identifier("c".to_string())),
                            operator: Token::Gte,
                            right: Box::new(ExpressionStatement::Num(2 as f64)),
                        }),
                        outcome: vec![Statement::Expression(ExpressionStatement::Prefix {
                            operator: Token::Bang,
                            right: Box::new(ExpressionStatement::Num(3 as f64)),
                        })],
                        alternate: vec![Statement::Expression(ExpressionStatement::Identifier(
                            "gg".to_string(),
                        ))],
                    })],
                })],
            },
            Testcase {
                name: "fn expression",
                input: String::from(
                    "let a = fn(b, c) {
                    let d = b + c
                    return d
                }",
                ),
                expected: vec![Statement::Let(
                    "a".to_string(),
                    Some(ExpressionStatement::Fn {
                        args: vec!["b".to_string(), "c".to_string()],
                        body: vec![
                            Statement::Let(
                                "d".to_string(),
                                Some(ExpressionStatement::Infix {
                                    left: Box::new(ExpressionStatement::Identifier(
                                        "b".to_string(),
                                    )),
                                    operator: Token::Plus,
                                    right: Box::new(ExpressionStatement::Identifier(
                                        "c".to_string(),
                                    )),
                                }),
                            ),
                            Statement::Return(Some(ExpressionStatement::Identifier(
                                "d".to_string(),
                            ))),
                        ],
                    }),
                )],
            },
            Testcase {
                name: "call expression",
                input: String::from("abc(def)"),
                expected: vec![Statement::Expression(ExpressionStatement::Call {
                    Caller: Box::new(ExpressionStatement::Identifier("abc".to_string())),
                    Args: vec![ExpressionStatement::Identifier("def".to_string())],
                })],
            },
        ];
        for testcase in testcases.into_iter() {
            let lexer = Lexer::new(&testcase.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            assert_eq!(program.statements.len(), testcase.expected.len());
            for (i, statement) in program.statements.into_iter().enumerate() {
                assert_eq!(statement, testcase.expected[i]);
            }
        }
    }
}
