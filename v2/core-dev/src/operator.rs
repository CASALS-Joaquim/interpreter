#![feature(impl_trait_in_bindings, if_let_guard)]

use std::convert::{ TryFrom, TryInto };
use core_stable::token;
use crate::ast;

macro_rules! op {
    { $left:ident = $right:ident($lhs:expr, $rhs:expr) } => {
        pub fn $left<'a>() -> (Operator<'a>, (u8, u8)) {
            (Operator::$right, ($lhs, $rhs))
        }
    };

    { $left:ident = $right:ident(_)($lhs:expr, $rhs:expr) } => {
        pub const $left: (fn(u16) -> Operator<'static>, (u8, u8)) = (|n| Operator::$right(n), ($lhs, $rhs));
    }
}

op! { ASSIGN = Assign(5, 4) }
op! { COMMA = Comma(_)(6, 7) }

op! { EQ = Eq(8, 9) }
op! { NOT_EQ = NotEq(8, 9) }

op! { LT = LT(10, 11) }
op! { GT = GT(10, 11) }
op! { GTE = GTE(10, 11) }
op! { LTE = LTE(10, 11) }

op! { PLUS = Plus(12, 13) }
op! { MINUS = Minus(12, 13) }

op! { MUL = Mul(14, 15) }
op! { DIV = Div(14, 15) }

op! { UNARY_PLUS = UnaryPlus(99, 18) }
op! { UNARY_MINUS = UnaryMinus(99, 18) }

op! { POW = Pow(17, 16) }

op! { NOT = Not(99, 19) }
op! { CALL = Call(22, 0) }
op! { INDEX = Index(22, 0) }

op! { MEMBER = Member(97, 98) }

op! { L_GROUPING = LParen(99, 0) }
op! { R_GROUPING = RParen(0, 100) }

pub const LITERAL: (fn(ast::Literal) -> Operator, (u8, u8)) = (|lit| Operator::Literal(lit), (99, 100));

#[derive(Debug, PartialEq, Clone)]
pub enum Operator<'a> {
    Assign,
    Comma(u16),
    Eq, NotEq,
    GT, LT, GTE, LTE,
    Plus, Minus,
    Mul, Div,
    UnaryPlus, UnaryMinus,
    Pow,
    Not,
    Call, Index,
    Member,
    List(u16), Map(u16),
    If(bool),
    LParen, RParen,
    Literal(ast::Literal<'a>)
}

impl<'a> Operator<'a> {
    pub fn number_of_parameters(self) -> u16 {
        use Operator::{
            Assign,
            Call,
            Comma,
            Div,
            Eq,
            Index,
            Minus,
            Mul,
            Not,
            NotEq,
            Plus,
            Pow,
            UnaryMinus,
            UnaryPlus,
            GT,
            GTE,
            LT,
            LTE,
            List,
            Map,
            If,
            Literal
        };

        match self {
            Assign => 2,
            Comma(n) => n,
            Eq | NotEq => 2,
            GT | LT | GTE | LTE => 2,
            Plus | Minus => 2,
            Mul | Div => 2,
            UnaryPlus | UnaryMinus => 1,
            Pow => 2,
            Not => 1,
            Call | Index => 2,
            List(n) | Map(n) => n,
            Literal(_) => 1
        }
    }
}



pub fn binding_power<'a> (
    op: Option<token::Token<'a>>,
    prefix: bool,
) -> Option<(Operator<'a>, (u8, u8))> {
    let op = op?.clone();
    let res = match op {
        token::Token::LeftParen => L_GROUPING(),
        token::Token::RightParen => R_GROUPING(),
        token::Token::Assign => ASSIGN(),
        token::Token::Plus if prefix => UNARY_PLUS(),
        token::Token::Minus if prefix => UNARY_MINUS(),
        token::Token::Plus | token::Token::Minus => PLUS(),
        token::Token::Asterisk | token::Token::Slash => MUL(),
        token::Token::Bang => NOT(),
        token::Token::Point => MEMBER(),
        literal if ast::Literal::try_from(literal).is_ok()  => (LITERAL.0(literal.try_into().unwrap()), LITERAL.1),
        _ => return None,
    };

    Some(res)
}

impl<'a> crate::Parse<'a> for (Operator<'a>, (u8, u8)) {
    fn parse(tokens: core_stable::token::Tokens<'a>) -> nom::IResult<core_stable::token::Tokens<'a>, Self> {
        use nom::{
            branch::alt,
            bytes::complete::tag
        };

        let op = alt((
            tag(crate::tokens![Plus]),
            tag(crate::tokens![Assign]),
            tag(crate::tokens![Comma]),
            tag(crate::tokens![Eq]),
            tag(crate::tokens![NotEq]),
            tag(crate::tokens![GT]), tag(crate::tokens![LT]), tag(crate::tokens![GTE]), tag(crate::tokens![LTE]),
            tag(crate::tokens![Plus], tag(crate::tokens![Minus]),
            tag(crate::tokens![Mul]), tag(crate::tokens![Div]),

            Pow,
            Not,
            Call, Index,
            Member,
            List(u16), Map(u16),
            If(bool),
            LParen, RParen,
            Literal(ast::Literal<'a>)
        ))(tokens)?;
    }
}
