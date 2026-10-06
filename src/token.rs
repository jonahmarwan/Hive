#[derive(Debug)]
pub struct Token {
    _type: String,
    value: String,
}

impl Token {
    pub fn new(_type: String, value: String) -> Token {
        let token = Token{_type, value };
        token
    }
}