#[cfg(test)]
mod test {
    use crate::tokenizer;
    use tokenizer::Token;

    #[test]
    fn test_tokenizer() {
        struct Testcase {
            input: String,
            expected: Vec<Token>,
        }
        let testcases = vec![
            Testcase {
                input: String::from("+-*/"),
                expected: vec![Token::Plus, Token::Minus, Token::Product, Token::Division],
            },
            Testcase {
                input: "SON   TUNG".to_string(),
                expected: vec![
                    Token::Ident(String::from("SON")),
                    Token::Ident(String::from("TUNG")),
                ],
            },
        ];

        for testcase in testcases.into_iter() {
            let mut lexer = tokenizer::Lexer::new(&testcase.input);
            let mut i = 0;
            loop {
                let token = lexer.next_token();
                if token == tokenizer::Token::Eof {
                    break;
                }
                assert_eq!(token, testcase.expected[i]);
                i += 1;
            }
        }
    }
}
