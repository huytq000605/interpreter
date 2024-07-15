mod evaluator;
mod lexer;
mod object;
mod parser;
mod statement;
mod token;

use std::io;

use lexer::Lexer;
use parser::{Parser, Program};

fn main() {
    print!("---huytq intepreter---");
    let evaluator = evaluator::Evaluator::new();
    let mut env = object::Environment::new(None);
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Err(e) => println!("{e}"),
            Ok(_) => {
                let lexer = Lexer::new(&input);
                let mut parser = Parser::new(lexer);
                let program = match parser.parse_program() {
                    Err(e) => {
                        println!("There was error during parsing, err={:?}", e);
                        Program { statements: vec![] }
                    }
                    Ok(program) => program,
                };
                match evaluator.eval(program, &mut env) {
                    Ok(v) => println!("{:?}", v),
                    Err(e) => println!("There was unexpected error, err={:?}", e),
                }
            }
        }
    }
}
