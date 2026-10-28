use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::position::Position;
use crate::codegen::Codegen;
use crate::symbol_table::{Symbol, SymbolTable};
use std::collections::HashMap;
use std::env;
use std::hash::Hash;

mod token;
mod lexer;
mod position;
mod keywords;
mod parser;
mod codegen;
mod ast;
mod runtime;
mod symbol_table;

fn main() {
    //Testing purposes
    let args: Vec<String> = env::args().collect();
    let mut lexer = Lexer{
        file_name: args[1].parse().unwrap(),
        position: Position{index: -1, line: 0, column: -1, file_name: "test.hv".parse().unwrap(), ftxt: "asdfasd".parse().unwrap() },
        current_char: None
    };

    let toks = lexer.make_token();
    
    // print statements are for debugging purposes
    
    //println!("{:?}", toks.clone());

    let mut p = Parser::new(toks);

    let stub = p.parse_statement().expect("failed to parse expression");
    // println!("{:?}", stub);

    let mut global_scope: HashMap<String, symbol_table::Symbol> = HashMap::new();
    let mut scope_stack = vec![global_scope];
    let mut symb_table = SymbolTable::new(scope_stack);


    Codegen::exec(&stub, symb_table);

}
