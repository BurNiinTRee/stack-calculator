//! This module contains [`Type`]s for simple integer arithmetic,
//! supporting addition, substraction, multiplication and division.
//!
//! [`Type`]: ../machine/trait.Type.html

use machine::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Integer(pub i64);

impl Value for Integer {
    fn apply(&self, _: &mut Stack) -> Result<(), Error> {
        Err(Error::NotCallable {
            token: Token::new(Integer(self.0)),
        })
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
/// A [`Type`] representing integers
///
/// [`Type`]: ../machine/trait.Type.html
pub struct IntegerMeta;

impl Type for IntegerMeta {
    fn parse_hint(&self) -> String {
        r"^\d+$".into()
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

impl fmt::Display for Addition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "+")
    }
}

#[derive(Debug)]
/// A [`Type`] capable of adding two [`Integer`]s
///
/// [`Type`]: ../machine/trait.Type.html
pub struct AdditionMeta;

impl Type for AdditionMeta {
    fn parse_hint(&self) -> String {
        r"^\+$".into()
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

impl fmt::Display for Substraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-")
    }
}

#[derive(Debug)]
/// A [`Type`] capable of substracting two [`Integer`]s
///
/// [`Type`]: ../machine/trait.Type.html
pub struct SubstractionMeta;

impl Type for SubstractionMeta {
    fn parse_hint(&self) -> String {
        r"^-$".into()
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

impl fmt::Display for Multiplication {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "*")
    }
}

#[derive(Debug)]
/// A [`Type`] capable of multiplying two [`Integer`]s
///
/// [`Type`]: ../machine/trait.Type.html
pub struct MultiplicationMeta;

impl Type for MultiplicationMeta {
    fn parse_hint(&self) -> String {
        r"^\*$".into()
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

impl fmt::Display for Division {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/")
    }
}

#[derive(Debug)]
/// A [`Type`] capable of dividing two [`Integer`]s
///
/// [`Type`]: ../machine/trait.Type.html
pub struct DivisionMeta;

impl Type for DivisionMeta {
    fn parse_hint(&self) -> String {
        r"^/$".into()
    }
    fn parse(&self, _: &str) -> Token {
        Token::new(Division)
    }
}
