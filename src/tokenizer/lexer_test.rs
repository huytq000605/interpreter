#[cfg(test)]
mod test {
    use crate::tokenizer;
    use tokenizer::lexer::{Lexer, Token};

    #[test]
    fn test_lexer() {
        struct Testcase {
            input: String,
            expected: Vec<Token>,
        }
        let testcases = vec![
            Testcase {
                input: String::from("+-*/==="),
                expected: vec![
                    Token::Plus,
                    Token::Minus,
                    Token::Asterisk,
                    Token::Slash,
                    Token::Equal,
                    Token::Assign,
                ],
            },
            Testcase {
                input: "SON   TUNG".to_string(),
                expected: vec![
                    Token::Ident(String::from("SON")),
                    Token::Ident(String::from("TUNG")),
                ],
            },
            Testcase {
                input: "12345".to_string(),
                expected: vec![Token::Int(12345)],
            },
            Testcase {
                input: "12345.456".to_string(),
                expected: vec![Token::Float(12345.456)],
            },
            Testcase {
                input: "let x = 5".to_string(),
                expected: vec![
                    Token::Let,
                    Token::Ident("x".to_string()),
                    Token::Assign,
                    Token::Int(5),
                ],
            },
        ];

        for testcase in testcases.into_iter() {
            let mut lexer = Lexer::new(&testcase.input);
            let mut i = 0;
            loop {
                let token = lexer.next_token();
                if token == Token::Eof {
                    break;
                }
                assert_eq!(token, testcase.expected[i]);
                i += 1;
            }
        }
    }
}
