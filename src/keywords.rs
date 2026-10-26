
//Number type
pub static DIGITS: &str = "0123456789";

//Letters-only type
pub static ALPHABETIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";

//Letters and Numbers type
pub static ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

//Keywords
pub static KEYWORDS: [&'static str; 8] = [
    "def",
    "let",
    "lambda",
    "func",
    "constant",
    "immutable",
    "mutable",
    "pure"
];

//Keywords type belonging to the list above
pub const T_KEYWORD: &str = "KEYWORD";

//Identifier type (any word that is not a keyword)
pub const T_IDENTIFIER: &str = "IDENTIFIER";

//Pi type
pub const T_PI: &str = "PI";

pub const T_NUMBER: &str = "NUMBER";