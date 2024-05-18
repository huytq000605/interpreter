mod lexer;
mod parser;
mod token;
mod statement;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let lexer = Lexer::new("let a = 5;");
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    println!("{:?}", program.statements.len());
    for statement in program.statements {
        println!("{:?}", statement.kind())
    }

    return;
}
