use std::fs;

use crate::position::Position;
use crate::runtime::RuntimeValue::{self, String};
use crate::token::{self, Node, Token};
use crate::keywords;
use crate::token::Token::{ASSIGNMENT, DIV, MINUS, MUL, PLUS, SEMICOLON, LQUOTATION, RQUOTATION};
use std::string;

//Defines what a Lexer looks like
pub struct Lexer {
    pub(crate) file_name: string::String,
    pub(crate) position: Position,
    pub(crate) current_char: Option<char>,
}

impl Lexer {
    //Move in Position and justify current_char to the new index

    pub fn text(&mut self) -> string::String {
        let t = fs::read_to_string(&self.file_name).unwrap_or_default();
        t
    }
    pub fn advance(&mut self){
        self.position.advance(self.current_char);
        if self.position.index < self.text().chars().count() as i32 {
            self.current_char = self.text().chars().nth(self.position.index as usize);
        } else {
            self.current_char = None;
        }
    }
    pub fn new_line(&mut self) {
        self.position.line += 1;
    }
    //Creates the list of Tokens
    pub fn make_token(&mut self) -> Vec<Node> {
        let mut tokens: Vec<Node> = Vec::new();
        let mut is_quote = false;
        self.advance();
        while self.current_char != None {
            if let Some(c) = self.current_char {
                match c {
                    '+' => {
                        tokens.push(Node::new(PLUS, RuntimeValue::String("+".to_string())));
                        self.advance();
                    },
                    '-' => {
                        tokens.push(Node::new(MINUS, RuntimeValue::String("-".to_string())));
                        self.advance();
                    },
                    '/' => {
                        tokens.push(Node::new(DIV, RuntimeValue::String("/".to_string())));
                        self.advance();
                    },
                    '*' => {
                        tokens.push(Node::new(MUL, RuntimeValue::String("*".to_string())));
                        self.advance();
                    },
                    '=' => {
                        tokens.push(Node::new(ASSIGNMENT, RuntimeValue::String("=".to_string())));
                        self.advance();
                    },
                    ';' => {
                        tokens.push(Node::new(SEMICOLON, RuntimeValue::String(";".to_string())));
                        self.advance();
                    },
                    '"' => {
                        if !is_quote {
                            is_quote = true;
                            tokens.push(Node::new(LQUOTATION, RuntimeValue::String('"'.to_string())));
                            self.advance();
                        } else {
                            is_quote = false;
                            tokens.push(Node::new(RQUOTATION, RuntimeValue::String('"'.to_string())));
                            self.advance(); 
                        }
                    },
                    _ if c.is_alphabetic() || c.is_ascii_digit() || c == '_' => tokens.push(self.read_identifier(is_quote)),
                    _ => {self.advance();}
                }
            }
        }
        tokens
    }
    //Reads an identifier composed of multiple characters
    pub fn read_identifier(&mut self, is_quote: bool) -> Node {
        let mut iden_string = RuntimeValue::String(string::String::from(""));
        // let pos_start = self.position.copy();
        let mut _type = Token::UNDEFINED;
        
        
        while let Some(c) = self.current_char {
        if c.is_alphanumeric() || c == '_' || (c.is_whitespace() && is_quote) {
            if let RuntimeValue::String(s) = &mut iden_string {
                s.push(c);
            }
            self.advance();
        } 
        else {
            break;
        }
    }

        if let RuntimeValue::String(s) = &mut iden_string {
            if keywords::KEYWORDS.contains(&s.as_str()) {
                _type = Token::KEYWORD;
            } else if s.chars().all(char::is_numeric) {
                _type = Token::NUMBER;
                iden_string = RuntimeValue::Int(s.parse().unwrap());
            } else {
                _type = Token::IDENTIFIER;
            }
    }
        Node::new(_type, iden_string)
    }
}
