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
    fn parse(&self, input: &str) -> Option<Token> {
        if input == "pop" {
            Some(Token::new(Pop))
        } else {
            None
        }
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
    fn parse(&self, input: &str) -> Option<Token> {
        input.parse().ok().map(|num| Token::new(Integer(num)))
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
    fn parse(&self, input: &str) -> Option<Token> {
        if input == "+" {
            Some(Token::new(Addition))
        } else {
            None
        }
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
    fn parse(&self, input: &str) -> Option<Token> {
        if input == "-" {
            Some(Token::new(Substraction))
        } else {
            None
        }
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
    fn parse(&self, input: &str) -> Option<Token> {
        if input == "*" {
            Some(Token::new(Multiplication))
        } else {
            None
        }
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
    fn parse(&self, input: &str) -> Option<Token> {
        if input == "/" {
            Some(Token::new(Division))
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use arithmetic::{Addition, Division, Integer, Multiplication, Substraction};
    use machine::{Stack, Token};
    #[test]
    fn simple_addition() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(4))).unwrap();
        s.push(Token::new(Integer(5))).unwrap();
        s.push(Token::new(Addition)).unwrap();
        assert_eq!(Integer(9), *s.pop().unwrap().downcast().unwrap());
    }
    #[test]
    fn simple_multiplication() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(4))).unwrap();
        s.push(Token::new(Integer(5))).unwrap();
        s.push(Token::new(Multiplication)).unwrap();
        assert_eq!(Integer(20), *s.pop().unwrap().downcast().unwrap());
    }
    #[test]
    fn simple_substraction() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(4))).unwrap();
        s.push(Token::new(Integer(5))).unwrap();
        s.push(Token::new(Substraction)).unwrap();
        assert_eq!(Integer(-1), *s.pop().unwrap().downcast().unwrap());
    }
    #[test]
    fn simple_division() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(20))).unwrap();
        s.push(Token::new(Integer(5))).unwrap();
        s.push(Token::new(Division)).unwrap();
        assert_eq!(Integer(4), *s.pop().unwrap().downcast().unwrap());
    }

}
