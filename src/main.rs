mod tokenizer;

use tokenizer::lexer::{Lexer, Token};

fn main() {
    let mut lexer = Lexer::new("let a = 5;");
    loop {
        let token = lexer.next_token();
        if token == Token::Eof {
            break;
        }
        println!("{:?}", token);
    }

    return;
}
