use std::collections::HashMap;
use std::cmp::PartialEq;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    /// Tokens definitions
    Illegal(char),
    EndOfFile,

    /// Identifiers + literals
    Ident(std::string::String),
    Int(isize),
    Float(f64),
    String(std::string::String),
    Boolean(bool),

    /// Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LowerThan,
    GreaterThan,
    LowerThanOrEqualTo,
    GreaterThanOrEqualTo,

    Eq,
    NotEq,

    /// Delimiters
    Comma,
    Semicolon,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    /// Keywords
    Function,
    Let,
    If,
    Else,
    Return,
    Unit,
}

pub use Token::*;

macro_rules! map {
    ( $( $x:expr => $y:expr),* ) => {
        {
            let mut temp_map: HashMap<&'static str, Token> = HashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

#[allow(dead_code)]
pub fn lookup_indent(ident: &str) -> Token {
    let keywords: HashMap<&str, Token> = map!{
        "fn" => Function,
        "let" => Let,
        "true" =>  Boolean(true),
        "false" => Boolean(false),
        "if" => If,
        "else" => Else,
        "return" => Return
    };
    match keywords.get(ident) {
        Some(value) => {
            value.clone()
        } None => {
            Ident(std::string::String::from(ident))
        }
    }
}