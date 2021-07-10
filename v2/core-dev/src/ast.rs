#![feature(new_zeroed)]
#![allow(dead_code)]

use std::convert::TryFrom;
use either::Either;
use core::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::collections::HashMap;
use std::cmp::PartialEq;
use core_stable::token;
use crate::operator::Operator;

#[derive(Debug, Clone)]
pub struct Identifier<'a>(pub Weak<RefCell<Context<'a>>>, pub usize);
#[derive(Debug, Clone, PartialEq)]
pub enum Ident<'a> {
    Checked(Identifier<'a>),
    UnChecked(&'a str)
}
impl PartialEq for Identifier<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.upgrade().unwrap() == other.0.upgrade().unwrap() && self.1 == other.1
    }
}

impl<'a> Identifier<'a> {
    pub fn get_name(&self) -> Option<&'a str> {
        self
            .0
            .upgrade()
            .unwrap()
            .borrow_mut()
            .idents
            .get(self.1)
            .map(|&str|str)
    }
}

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub idents: Vec<&'a str>,
    pub parent: Option<Weak<RefCell<Context<'a>>>>,
    pub zone: Rc<Vec<Statement<'a>>>
}

impl<'a> Context<'a> {
    pub fn find_named_ident(this: Rc<RefCell<Context<'a>>>, identifier: &'a str) -> Option<Identifier<'a>> {
        let ident = this
            .borrow_mut()
            .idents
            .iter()
            .enumerate()
            .skip_while(|(_, &ident)| ident == identifier)
            .last()
            .map(|(index, &ident)| (index, ident));
        Some(
            ident
            .map(|(index, _)| Identifier(Rc::downgrade(&this), index)))
            .unwrap_or_else(move || 
                this
                .borrow_mut()
                .parent
                .as_ref()
                .map(|parent| Self::find_named_ident(parent.upgrade().unwrap(), identifier)).unwrap_or(None))
    }

    pub fn register_named(this: Rc<RefCell<Context<'a>>>, identifier: &'a str) -> Identifier<'a> {
        this.borrow_mut().idents.push(identifier);
        this
            .borrow_mut()
            .idents
            .iter()
            .enumerate()
            .last()
            .map(|(index, _)| Identifier(Rc::downgrade(&this), index))
            .unwrap()
    }
}

impl<'a> PartialEq for Context<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.zone == other.zone
    }
}

impl<'ctx> Context<'ctx> {
    fn new(statements: &Rc<Vec<Statement<'ctx>>>) -> Self {
        Self {
            idents: Vec::with_capacity(255),
            parent: None,
            zone: Rc::clone(statements)
        }
    }
}

impl<'ctx> Default for Context<'ctx> {
    fn default() -> Self {
        Self {
            idents: Vec::with_capacity(255),
            parent: None,
            zone: Rc::new(Vec::new())
        }
    }
}

pub struct Function<'ctx>(Context<'ctx>, &'ctx [&'ctx str]);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    /// Identifiers + literals
    Ident(Either<Identifier<'a>, &'a str>),
    Int(isize),
    Float(f64),
    String(&'a str),
    Boolean(bool),
}

impl<'a> From<isize> for Literal<'a> {
    fn from(int: isize) -> Self {
        Self::Int(int)
    }
}

impl<'a> From<f64> for Literal<'a> {
    fn from(float: f64) -> Self {
        Self::Float(float)
    }
}

impl<'a> From<bool> for Literal<'a> {
    fn from(boolean: bool) -> Self {
        Self::Boolean(boolean)
    }
}

impl<'a> From<&'a str> for Literal<'a> {
    fn from(str: &'a str) -> Self {
        Self::String(str)
    }
}

impl<'a> From<Identifier<'a>> for Literal<'a> {
    fn from(ident: Identifier<'a>) -> Self {
        Self::Ident(either::Left(ident))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Literal(Literal<'a>),
    Operator(Operator<'a>),
    Block(Context<'a>)
}



impl<'a> From<Context<'a>> for Expression<'a> {
    fn from(block: Context<'a>) -> Self {
        Self::Block(block)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement<'a> {
    Let(LetStatement<'a>),
    Return(ReturnStatement<'a>),
    Expr(Expression<'a>)
}
impl<'a> TryFrom<token::Token<'a>> for Literal<'a> {
    type Error = token::Token<'a>;
    fn try_from(tok: token::Token<'a>) -> std::result::Result<Self, token::Token<'a>> {
        use token::Token::{Boolean, Float, Int, String, Ident};
        match tok {
            Ident(ident) => Ok(Self::Ident(Either::Right(ident))),
            Int(int) => Ok(Self::Int(int)),
            Float(float) => Ok(Self::Float(float)),
            String(str) => Ok(Self::String(str)),
            Boolean(bool) => Ok(Self::Boolean(bool)),
            tok => Err(tok)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement<'a> {
    ident: Ident<'a>,
    value: Box<Expression<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnStatement<'a> {
    value: Box<Expression<'a>>,
}