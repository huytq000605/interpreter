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
                Err(e) => program.errors.push(e)
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
                Err(e) => Err(e)
            },
            Token::Return => match self.parse_return_statement() {
                Ok(statement) => Ok(statement),
                Err(e) => Err(e)
            },
            _ => match self.parse_expression_statement(PRECEDENCE_LOWEST) {
                Ok(statement) => Ok(Statement::Expression(statement)),
                Err(e) => Err(e)
            }
        }
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

    fn parse_let_statement(&mut self) -> Result<Statement, String> {
        // Skip through let token
        self.next_token();
        let literal = match &self.cur_token {
            Token::Ident(literal) => literal.clone(),
            _ => return Err("Invalid let statement".to_string()),
        };
        if self.peek_token != Token::Assign {
            return Ok(Statement::Let {
                identifier: literal,
                value: None,
            });
        }

        // Skip through identifier token
        self.next_token();
        // Skip through assign token
        self.next_token();

        let let_statement = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => Statement::Let {
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

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        // Skip through return token
        self.next_token();
        match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => Ok(Statement::Return {
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
            Token::Bang | Token::Minus => {
                let right = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
                    Ok(statement) => statement,
                    Err(e) => return Err(e),
                };
                ExpressionStatement::PrefixExpression {
                    operator: self.cur_token.clone(),
                    right: Box::new(right),
                }
            }
            Token::Num(num) => ExpressionStatement::Num(*num),
            Token::Ident(literal) => ExpressionStatement::Identifier(literal.clone()),
            Token::If => {
                match self.parse_if_expression() {
                    Ok(statement) => statement,
                    Err(e) => return Err(e)
                }
            }
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
                _ => return Ok(left),
            };
        }

        Ok(left)
    }

    fn parse_if_expression(&mut self) -> Result<ExpressionStatement, String>{
        // Skip through IF token
        self.next_token();

        let mut has_lparen = false;
        if self.peek_token == Token::LParen {
            has_lparen = true;
            self.next_token();
        }
        let condition = match self.parse_expression_statement(PRECEDENCE_LOWEST) {
            Ok(statement) => statement,
            Err(e) => return Err(e)
        };
        if has_lparen {
            if self.peek_token != Token::RParen {
                return Err(format!("Expected RParen, got = {:?}", self.peek_token))
            }
            // Skip through RParen
            self.next_token()
        }

        if self.peek_token != Token::LBracket {
            return Err(format!("Expected LBracket, got = {:?}", self.peek_token))
        }

        // Skip through LBracket
        self.next_token();

        // Start parsing outcome until facing RBracket
        let mut outcome = vec![];
        while self.cur_token != Token::RBracket {
            let statement = match self.parse_statement() {
                Ok(statement) => statement,
                Err(e) => return Err(e)
            };
            outcome.push(statement)
        }

        // Skip through RBracket
        self.next_token();

        if self.cur_token != Token::Else {
            Ok(ExpressionStatement::If{
                condition: Box::new(condition),
                outcome,
                alternate: vec![]
            })
        } else {
            // TODO: implement else part
            return Ok(ExpressionStatement::If{
                condition: Box::new(condition),
                outcome,
                alternate: vec![]
            })
        }

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
                expected: vec![Statement::Let {
                    identifier: "a".to_string(),
                    value: None,
                }],
            },
            Testcase {
                name: "let and return",
                input: String::from(
                    "let a = 6;
                    return 5",
                ),
                expected: vec![
                    Statement::Let {
                        identifier: "a".to_string(),
                        value: Some(ExpressionStatement::Num(6 as f64)),
                    },
                    Statement::Return {
                        value: Some(ExpressionStatement::Num(5 as f64)),
                    },
                ],
            },
            Testcase {
                name: "let and expression parsing",
                input: String::from("let a = 5+6+7"),
                expected: vec![Statement::Let {
                    identifier: "a".to_string(),
                    value: Some(ExpressionStatement::Infix {
                        left: Box::new(ExpressionStatement::Infix {
                            left: Box::new(ExpressionStatement::Num(5 as f64)),
                            operator: Token::Plus,
                            right: Box::new(ExpressionStatement::Num(6 as f64)),
                        }),
                        operator: Token::Plus,
                        right: Box::new(ExpressionStatement::Num(7 as f64)),
                    }),
                }],
            },
            Testcase {
                name: "let and expression with different precedence check",
                input: String::from("let a = 5 + 6 / 7"),
                expected: vec![Statement::Let{
                    identifier: "a".to_string(),
                    value: Some(ExpressionStatement::Infix {
                        left: Box::new(ExpressionStatement::Num(5 as f64)),
                        operator: Token::Plus,
                        right: Box::new(ExpressionStatement::Infix {
                            left: Box::new(ExpressionStatement::Num(6 as f64)),
                            operator: Token::Slash,
                            right: Box::new(ExpressionStatement::Num(7 as f64)),
                        }),
                    }),
                }],
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
