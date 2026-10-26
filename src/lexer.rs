use crate::position::Position;
use crate::token::{Node, Token};
use crate::keywords;
use crate::keywords::KEYWORDS;
use crate::token::Token::{DIV, MINUS, MUL, PLUS};

//Defines what a Lexer looks like
pub struct Lexer {
    pub(crate) file_name: String,
    pub(crate) text: String,
    pub(crate) position: Position,
    pub(crate) current_char: Option<char>,
}

impl Lexer {
    //Move in Position and justify current_char to the new index
    pub fn advance(&mut self){
        self.position.advance(self.current_char);
        if self.position.index < self.text.chars().count() as i32 {
            self.current_char = self.text.chars().nth(self.position.index as usize);
        } else {
            self.current_char = None;
        }
    }
    //Creates the list of Tokens
    pub fn make_token(&mut self) -> Vec<Node> {
        let mut tokens: Vec<Node> = Vec::new();
        self.advance();
        while self.current_char != None {
            if let Some(c) = self.current_char {
                match c {
                    '+' => tokens.push(Node::new(PLUS, "+".to_string())),
                    '-' => tokens.push(Node::new(MINUS, "-".to_string())),
                    '/' => tokens.push(Node::new(DIV, "/".to_string())),
                    '*' => tokens.push(Node::new(MUL, "*".to_string())),
                    _ if c.is_alphabetic() || c.is_ascii_digit() || c == '_' => tokens.push(self.read_identifier()),
                    _ => {}
                }
                self.advance();
            }
        }
        tokens
    }
    //Reads an identifier composed of multiple characters
    pub fn read_identifier(&mut self) -> Node {
        let mut iden_string = String::new();
        // let pos_start = self.position.copy();
        let mut _type = Token::UNDEFINED;

        while self.current_char != None && self.current_char.unwrap_or(' ').is_alphanumeric() || self.current_char.unwrap_or(' ') == '_' {
            iden_string.push(self.current_char.unwrap());
            self.advance();
        }

        if keywords::KEYWORDS.contains(&iden_string.as_str()) {
            _type = Token::KEYWORD;
        } else if iden_string.chars().all(char::is_numeric) {
            _type = Token::NUMBER;
        } else {
            _type = Token::IDENTIFIER;
        }
        Node::new(_type, iden_string)
    }
}
