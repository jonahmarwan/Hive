pub static DIGITS: &str = "0123456789";
pub static ALPHABETIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub static KEYWORDS: [&'static str; 7] = [
    "def",
    "let",
    "lambda",
    "func",
    "constant",
    "immutable",
    "mutable"
];

pub static T_KEYWORD: &str = "KEYWORD";
pub static T_IDENTIFIER: &str = "IDENTIFIER";

pub static T_PI: &str = "PI";