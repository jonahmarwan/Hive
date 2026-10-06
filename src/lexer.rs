use crate::position::Position;
use crate::token::Token;
use crate::keywords;
pub struct Lexer {
    pub(crate) file_name: String,
    pub(crate) text: String,
    pub(crate) position: Position,
    pub(crate) current_char: Option<char>,
}

impl Lexer {
    pub fn advance(&mut self){
        self.position.advance(self.current_char);
        if self.position.index < self.text.chars().count() as i32 {
            self.current_char = self.text.chars().nth(self.position.index as usize);
        } else {
            self.current_char = None;
        }
    }
    pub fn make_token(&mut self){
        let mut tokens: Vec<Token> = Vec::new();
        self.advance();
        while self.current_char != None {
            match self.current_char {
                Some(c) => {
                    if keywords::DIGITS.contains(c) {
                        tokens.push(Token::new("number".parse().unwrap(), c.to_string()));
                        self.advance();
                    } else if c.is_alphanumeric() || c == '_' {
                        tokens.push(self.read_identifier());
                        self.advance();
                    }
                }
                None => {}
            };
        }
        println!("{:?}", tokens);
    }
    pub fn read_identifier(&mut self) -> Token {
        let mut iden_string = String::new();
        // let pos_start = self.position.copy();
        let mut _type = "";

        while self.current_char != None && self.current_char.unwrap_or(' ').is_alphanumeric() || self.current_char.unwrap_or(' ') == '_' {
            iden_string.push(self.current_char.unwrap());
            self.advance();
        }

        if keywords::KEYWORDS.contains(&iden_string.as_str()) {
            _type = keywords::T_KEYWORD;
        } else {
            _type = keywords::T_IDENTIFIER;
        }
        Token::new(_type.to_string(), iden_string)
    }
}
