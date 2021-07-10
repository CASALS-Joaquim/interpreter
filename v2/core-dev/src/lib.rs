extern crate core_stable;

#[allow(unused_imports)]
pub use core_stable::token;



#[allow(unused_imports)]
pub use core_stable::lexer;

#[allow(unused_imports)]
#[macro_use]
pub mod ast;

#[allow(unused_imports)]
pub mod parser;

#[allow(unused_imports)]
pub mod operator;

pub mod utils;

#[macro_export] macro_rules! tokens {
    [$($tokens:ident$(($value:expr))?),*] => {
        token::Tokens(&[$(token::Token::$tokens$(($value))?)*])
    }
}

pub trait Parse<'a, Output: Parse<'a> = Self> {
    fn parse(tokens: token::Tokens<'a>) -> nom::IResult<token::Tokens<'a>, Output>;
}