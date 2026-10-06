use crate::lexer::Lexer;
use crate::position::Position;

mod token;
mod lexer;
mod position;
mod keywords;

fn main() {
    let mut lexer = Lexer{
        file_name: "sdfs".parse().unwrap(),
        text: "let hi1 wtvthefuck".parse().unwrap(),
        position: Position{index: -1, line: 0, column: -1, file_name: "asdad".parse().unwrap(), ftxt: "asdfasd".parse().unwrap() },
        current_char: None
    };
    lexer.make_token();
}
