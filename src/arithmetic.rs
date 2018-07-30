//! This module contains [`MetaObject`]s for simple integer arithmetic,
//! supporting addition, substraction, multiplication and division.
//!
//! [`MetaObject`]: ../machine/trait.MetaObject.html

use machine::*;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Integer(pub i64);

impl Value for Integer {
    fn apply(&self, _: &mut Stack) -> Result<(), Error> {
        Err(Error::NotCallable {
            token: Token::new(Integer(self.0)),
        })
    }
}

#[derive(Debug)]
/// A [`MetaObject`] representing integers
///
/// [`MetaObject`]: ../machine/trait.MetaObject.html
pub struct IntegerMeta;

impl MetaObject for IntegerMeta {
    fn parse_hint(&self) -> Regex {
        Regex::new(r"^\d+$").unwrap()
    }
    fn parse(&self, input: &str) -> Token {
        Token::new(Integer(input.parse().unwrap()))
    }
}

#[derive(Debug)]
struct Addition;

impl Value for Addition {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error> {
        let bt = stack.pop()?;
        let at = stack.pop()?;

        let Integer(a): Integer = at
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: bt })?;

        stack.push(Token::new(Integer(a + b)))?;
        Ok(())
    }
}

#[derive(Debug)]
/// A [`MetaObject`] capable of adding two [`Integer`]s
///
/// [`MetaObject`]: ../machine/trait.MetaObject.html
pub struct AdditionMeta;

impl MetaObject for AdditionMeta {
    fn parse_hint(&self) -> Regex {
        Regex::new(r"^\+$").unwrap()
    }
    fn parse(&self, _: &str) -> Token {
        Token::new(Addition)
    }
}

#[derive(Debug)]
struct Substraction;

impl Value for Substraction {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error> {
        let bt = stack.pop()?;
        let at = stack.pop()?;

        let Integer(a): Integer = at
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: bt })?;

        stack.push(Token::new(Integer(a - b)))?;
        Ok(())
    }
}
#[derive(Debug)]
/// A [`MetaObject`] capable of substracting two [`Integer`]s
///
/// [`MetaObject`]: ../machine/trait.MetaObject.html
pub struct SubstractionMeta;

impl MetaObject for SubstractionMeta {
    fn parse_hint(&self) -> Regex {
        Regex::new(r"^-$").unwrap()
    }
    fn parse(&self, _: &str) -> Token {
        Token::new(Substraction)
    }
}

#[derive(Debug)]
struct Multiplication;

impl Value for Multiplication {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error> {
        let bt = stack.pop()?;
        let at = stack.pop()?;

        let Integer(a): Integer = at
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: bt })?;

        stack.push(Token::new(Integer(a * b)))?;
        Ok(())
    }
}
#[derive(Debug)]
/// A [`MetaObject`] capable of multiplying two [`Integer`]s
///
/// [`MetaObject`]: ../machine/trait.MetaObject.html
pub struct MultiplicationMeta;

impl MetaObject for MultiplicationMeta {
    fn parse_hint(&self) -> Regex {
        Regex::new(r"^\*$").unwrap()
    }
    fn parse(&self, _: &str) -> Token {
        Token::new(Multiplication)
    }
}
#[derive(Debug)]
struct Division;

impl Value for Division {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error> {
        let bt = stack.pop()?;
        let at = stack.pop()?;

        let Integer(a): Integer = at
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongType { token: bt })?;

        stack.push(Token::new(Integer(a / b)))?;
        Ok(())
    }
}
#[derive(Debug)]
/// A [`MetaObject`] capable of dividing two [`Integer`]s
///
/// [`MetaObject`]: ../machine/trait.MetaObject.html
pub struct DivisionMeta;

impl MetaObject for DivisionMeta {
    fn parse_hint(&self) -> Regex {
        Regex::new(r"^/$").unwrap()
    }
    fn parse(&self, _: &str) -> Token {
        Token::new(Division)
    }
}
