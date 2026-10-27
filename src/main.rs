use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::position::Position;
use crate::codegen::Codegen;
use std::env;

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
    let args: Vec<String> = env::args().collect();
    let mut lexer = Lexer{
        file_name: args[1].parse().unwrap(),
        position: Position{index: -1, line: 0, column: -1, file_name: "test.hv".parse().unwrap(), ftxt: "asdfasd".parse().unwrap() },
        current_char: None
    };

    let toks = lexer.make_token();
    println!("{:?}", toks.clone());
    let mut p = Parser::new(toks);

    let stub = p.parse_expression().expect("failed to parse expression");

    Codegen::exec(&stub);
}
