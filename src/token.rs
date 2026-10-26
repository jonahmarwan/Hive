
// Define what a Token looks like

use crate::keywords;

#[derive(Debug, Clone)]
pub struct Node {
    pub _type: Token,
    pub value: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Token{
    PLUS,
    MINUS,
    EOF,
    IDENTIFIER,
    KEYWORD,
    NUMBER,
    MUL,
    DIV,
    UNDEFINED
}

impl Node {
    //Create a new Token
    pub fn new(_type: Token, value: String) -> Node {
        let token = Node {_type, value };
        token
    }
}