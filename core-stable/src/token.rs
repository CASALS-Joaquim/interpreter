use std::collections::HashMap;

pub struct Token {
    pub token_type: String,
    pub literal: String,
}

macro_rules! map {
    ( $( $x:expr => $y:expr),* ) => {
        {
            let mut temp_map: HashMap<&'static str, &'static str> = HashMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

#[allow(dead_code)]
pub mod tokens {
    /// Tokens definitions
    pub const ILLEGAL: &'static str = "ILLEGAL";
    pub const EOF: &'static str = "EOF";

    /// Identifiers + literals
    pub const IDENT: &'static str = "IDENT";
    pub const INT: &'static str = "INT";

    /// Operators
    pub const ASSIGN: &'static str = "=";
    pub const PLUS: &'static str = "+";
    pub const MINUS: &'static str = "-";
    pub const BANG: &'static str = "!";
    pub const ASTERISK: &'static str = "*";
    pub const SLASH: &'static str = "/";

    pub const LT: &'static str = "<";
    pub const GT: &'static str = ">";

    pub const EQ: &'static str = "==";
    pub const NOT_EQ: &'static str = "!=";

    /// Delimiters
    pub const COMMA: &'static str = ",";
    pub const SEMICOLON: &'static str = ";";

    pub const LPAREN: &'static str = "(";
    pub const RPAREN: &'static str = ")";
    pub const LBRACE: &'static str = "{";
    pub const RBRACE: &'static str = "}";

    /// Keywords
    pub const FUNCTION: &'static str = "FUNCTION";
    pub const LET: &'static str = "LET";
    pub const TRUE: &'static str =  "TRUE";
    pub const FALSE: &'static str = "FALSE";
    pub const IF: &'static str = "IF";
    pub const ELSE: &'static str = "ELSE";
    pub const RETURN: &'static str = "RETURN";
}

pub use self::tokens::{
    ASSIGN,
    ASTERISK,
    BANG,
    COMMA,
    ELSE,
    EOF,
    EQ,
    FALSE,
    FUNCTION,
    GT,
    IDENT,
    IF,
    ILLEGAL,
    INT,
    LBRACE,
    LET,
    LPAREN,
    LT,
    MINUS,
    NOT_EQ,
    PLUS,
    RBRACE,
    RETURN,
    RPAREN,
    SEMICOLON,
    SLASH,
    TRUE
};

#[allow(dead_code)]
pub fn lookup_indent(ident: &str) -> &str {
    let keywords: HashMap<&str, &str> = map!{
        "fn" => FUNCTION,
        "let" => LET,
        "true" =>  TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN
    };
    match keywords.get(ident) {
        Some(value) => {
            value
        } None => {
            IDENT
        }
    }
}