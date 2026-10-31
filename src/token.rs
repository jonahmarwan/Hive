use crate::runtime::RuntimeValue;


// Define what a Token looks like
#[derive(Debug, Clone)]
pub struct Node {
    pub _type: Token,
    pub value: RuntimeValue,
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
    ASSIGNMENT,
    UNDEFINED,
    SEMICOLON,
    LQUOTATION,
    RQUOTATION
}

impl Node {
    //Create a new Token
    pub fn new(_type: Token, value: RuntimeValue) -> Node {
        let token = Node {_type, value };
        token
    }
}