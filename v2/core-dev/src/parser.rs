#![allow(dead_code)]
use crate::operator;
use std::iter::FusedIterator;
use std::cell::RefCell;
use crate::{ token, tokens };
use std::any::Any;
use std::rc::{ Rc, Weak };
use std::str::FromStr;

use crate::ast;
use core_stable::token::{Token, Tokens};
use core_stable::lexer;
use crate::operator as op;

pub struct Parser<'a> {
    ctx: Vec<Rc<RefCell<ast::Context<'a>>>>
}

impl<'a> Parser<'a> {
    fn let_statement(&mut self, tokens: token::Tokens<'a>) -> nom::IResult<token::Tokens<'a>, ast::Polish<'a>> {
        dbg!(&tokens);
        let (tokens, _) = tag(tokens![Let])(tokens)?;dbg!(&tokens);
        let (tokens, ident) = tag(tokens![Ident("test")])(tokens)?;dbg!(&tokens);
        let ident = match ident {
            token::Tokens(&[token::Token::Ident(ident)]) => ident,
            _ => unreachable!()
        };
        let (tokens, _) = tag(tokens![Assign])(tokens)?;dbg!(&tokens);
        let (tokens, value) = self.literal(tokens)?;dbg!(&tokens);
        let (tokens, _) = tag(tokens![Semicolon])(tokens)?;dbg!(&tokens);
        let (tokens, _) = opt(eof)(tokens)?;
        dbg!(ident, &value);
        Ok(
            (
                tokens, ast::Statement::Let(
                    ast::LetStatement {
                        ident: ast::Ident::UnChecked(ident),
                        value
                    }
                )
            )
        )
    }

    fn literal(&mut self, tokens: token::Tokens<'a>) -> nom::IResult<token::Tokens<'a>, ast::Literal<'a>> {
        let (tokens, value) = alt((
            tag(tokens![Int(0)]),
            tag(tokens![Float(0.)]),
            tag(tokens![String("")]),
            tag(tokens![Boolean(false)]),
            tag(tokens![Ident("")]),
        ))(tokens)?;
        let value = match value.0[0] {
            token::Token::Ident(ident) => ast::Literal::from(ident),
            token::Token::Int(value) => ast::Literal::from(value),
            /*token::Token::Float(value) => ast::Literal::from(value),*/
            token::Token::String(str) => ast::Literal::from(str),
            token::Token::Boolean(bool) => ast::Literal::from(bool),
            _ => unreachable!()
        };
        dbg!(&tokens, &value);
        Ok((tokens, value))
    }

    fn get_identifier(&mut self, tokens: token::Tokens<'a>) -> nom::IResult<token::Tokens<'a>, Option<ast::Identifier<'a>>> {
        let (tokens, ident) = tag(tokens![Ident("")])(tokens)?;
        let ident = match ident {
            token::Tokens(&[token::Token::Ident(ident)]) =>
                ast::Context::<'a>::find_named_ident(self.ctx.last().map(|ctx| Rc::clone(ctx)).unwrap(), ident),
            _ => unreachable!(),
        };
        Ok((tokens, ident))
    }
}

struct Frame<'a> {
    min_bp: u8,
    lhs: Option<ast::Expression<'a>>,
    token: Option<char>,
}

fn expr_bp<'a>(lexer: token::Tokens<'a>) -> Option<&'a ast::Expression<'a>> {
    let mut top = Frame {
        min_bp: 0,
        lhs: None,
        token: None,
    };
    let mut stack = Vec::new();

    loop {
        let token = lexer.next();
        let (token, r_bp) = loop {
            match operator::binding_power(token, top.lhs.is_none()) {
                Some((t, (l_bp, r_bp))) if top.min_bp <= l_bp =>{
                    break (t, r_bp)
                }
                _ => {
                    let res = top;
                    top = match stack.pop() {
                        Some(it) => it,
                        None => {
                            eprintln!();
                            return res.lhs;
                        }
                    };

                    let mut args = Vec::new();
                    args.extend(top.lhs);
                    args.extend(res.lhs);
                    let token = res.token.unwrap();
                    eprint!("{} ", token);
                    top.lhs = Some(S::Cons(token, args));
                }
            };
        };

        if token == ')' {
            assert_eq!(top.token, Some('('));
            let res = top;
            top = stack.pop().unwrap();
            top.lhs = res.lhs;
            continue;
        }

        stack.push(top);
        top = Frame {
            min_bp: r_bp,
            lhs: None,
            token: Some(token),
        };
    }
}



#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use core_stable::lexer;
    use super::{
        Parser,
        token,
        ast
    };
    use either::Either;

    #[test]
    fn test_let_statement<'a>() {
        let input = "let test = 5;";
        let lex = &lexer::new(input).collect::<Vec<_>>()[..];
        let lex = token::Tokens(lex);
        let mut pars = super::Parser {
            ctx: vec![Rc::new(RefCell::new(ast::Context::default()))]
        };
        let expected_ast = ast::Polish::new(vec![
            Either::Left(ast::Statement::Let{}),
            Either::Right(ast::Expression::Calculate(Either::Right(ast::Literal::Ident(Either::Right("test"))))),
            Either::Right(ast::Expression::Calculate(Either::Right(ast::Literal::from(5))))
        ]);
        let result_ast = match pars.let_statement(lex) {
            Ok((_, result_ast)) => result_ast,
            Err(err) => {
                dbg!(err);
                unreachable!();
            }
        };
        dbg!(&result_ast);
        dbg!(&expected_ast);
        assert_eq!(expected_ast, result_ast);
    }
}