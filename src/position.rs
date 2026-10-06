
pub struct Position {
    pub index: i32,
    pub(crate) line: i32,
    pub(crate) column: i32,
    pub(crate) file_name: String,
    pub(crate) ftxt: String
}

impl Position{
    pub fn advance(&mut self, current_char: Option<char>) {
        self.index += 1;
        self.column += 1;

        if current_char.unwrap_or('a') == '\n' {
            self.line += 1;
            self.column = 0;
        }
    }
    pub fn copy(&mut self) -> Position {
        return Position{
            index: self.index,
            line: self.line,
            column: self.column,
            file_name: self.file_name.clone(),
            ftxt: self.ftxt.clone()
        };
    }
}
