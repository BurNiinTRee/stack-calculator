//! This module contains [`Parser`]s for simple integer arithmetic,
//! supporting addition, substraction, multiplication and division.
//!
//! [`Parser`]: ../machine/trait.Parser.html

use machine::*;
use std::fmt;

pub fn arithmetic_module() -> ParserAggregator {
    let mut parser = ParserAggregator::new();
    parser.push(AdditionMeta);
    parser.push(SubstractionMeta);
    parser.push(MultiplicationMeta);
    parser.push(DivisionMeta);
    parser.push(IntegerMeta);
    parser.push(PopMeta);
    parser
}

#[derive(Debug)]
pub struct Pop;

impl Value for Pop {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error> {
        stack.pop()?;
        Ok(())
    }
}

impl fmt::Display for Pop {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct PopMeta;

impl Parser for PopMeta {
    fn parse_hint(&self) -> String {
        r"pop".into()
    }

    fn parse(&self, _: &str) -> Token {
        Token::new(Pop)
    }
}

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
/// A [`Parser`] representing integers
///
/// [`Parser`]: ../machine/trait.Parser.html
pub struct IntegerMeta;

impl Parser for IntegerMeta {
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
            .ok_or(Error::WrongParser { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongParser { token: bt })?;

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
/// A [`Parser`] capable of adding two [`Integer`]s
///
/// [`Parser`]: ../machine/trait.Parser.html
pub struct AdditionMeta;

impl Parser for AdditionMeta {
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
            .ok_or(Error::WrongParser { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongParser { token: bt })?;

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
/// A [`Parser`] capable of substracting two [`Integer`]s
///
/// [`Parser`]: ../machine/trait.Parser.html
pub struct SubstractionMeta;

impl Parser for SubstractionMeta {
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
            .ok_or(Error::WrongParser { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongParser { token: bt })?;

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
/// A [`Parser`] capable of multiplying two [`Integer`]s
///
/// [`Parser`]: ../machine/trait.Parser.html
pub struct MultiplicationMeta;

impl Parser for MultiplicationMeta {
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
            .ok_or(Error::WrongParser { token: at })?;
        let Integer(b): Integer = bt
            .downcast_ref()
            .cloned()
            .ok_or(Error::WrongParser { token: bt })?;

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
/// A [`Parser`] capable of dividing two [`Integer`]s
///
/// [`Parser`]: ../machine/trait.Parser.html
pub struct DivisionMeta;

impl Parser for DivisionMeta {
    fn parse_hint(&self) -> String {
        r"^/$".into()
    }
    fn parse(&self, _: &str) -> Token {
        Token::new(Division)
    }
}
