use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::position::Position;

mod token;
mod lexer;
mod position;
mod keywords;
mod parser;
mod codegen;
mod ast;
mod runtime;

fn main() {
    //Testing purposes
    let mut lexer = Lexer{
        file_name: "sdfs".parse().unwrap(),
        text: "1 + 1 + 2".parse().unwrap(),
        position: Position{index: -1, line: 0, column: -1, file_name: "asdad".parse().unwrap(), ftxt: "asdfasd".parse().unwrap() },
        current_char: None
    };
    let toks = lexer.make_token();

    let mut p = Parser::new(toks);

    let stub = p.parse_expression();
    println!("{:?}", stub)
}
