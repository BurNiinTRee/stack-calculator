use downcast_rs::Downcast;
use regex::Regex;

#[derive(Debug, Fail)]
/// Represents an Error
pub enum Error {
    /// This Variant is to be given, when an Operator has too few Values on the Stack
    #[fail(display = "not enough values on the stack")]
    NotEnoughValues,
    /// This Variant is to be given, when an Operator encounters a Token of the wrong type on the Stack
    #[fail(display = "token is not the expected type: {:?}", token,)]
    WrongParser { token: Token },
    /// This Variant is to be given, when an Operator doesn't modify the Stack, but should be pushed as is
    #[fail(display = "token is not an operator: {:?}", token)]
    NotCallable { token: Token },
}

// A Token, as it is found on the Stack
#[derive(Debug)]
pub struct Token(Box<dyn Value>);

impl Token {
    /// Creates a new Token from a value, that implements [`Value`]
    pub fn new<T: Value + 'static>(value: T) -> Token {
        Token(Box::new(value))
    }
    /// Tries to get the underlying [`Value`] of the desired type
    pub fn downcast<T: Value + 'static>(self) -> Option<Box<T>> {
        self.0.downcast().ok()
    }
    /// Tries to get a reference to the underlying [`Value`] of the desired type
    pub fn downcast_ref<T: Value + 'static>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

impl ::std::fmt::Display for Token {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self.0, f)
    }
}

/// A Stack on which [`Token`]s can be pushed on.
#[derive(Debug)]
pub struct Stack(Vec<Token>);

impl Stack {
    /// Creates a new empty [`Stack`]
    pub fn new() -> Stack {
        Stack(Vec::new())
    }
    /// Pushes a [`Token`] onto the Stack, and tries to apply it
    pub fn push(&mut self, t: Token) -> Result<(), Error> {
        match t.0.apply(self) {
            Err(Error::NotCallable { token }) => Ok(self.0.push(token)),
            n => n,
        }
    }
    /// Pops the last [`Token`] from the [`Stack`]
    pub fn pop(&mut self) -> Result<Token, Error> {
        self.0.pop().ok_or(Error::NotEnoughValues)
    }
}

impl ::std::fmt::Display for Stack {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for t in &self.0 {
            write!(f, "{} ", t)?;
        }
        Ok(())
    }
}

/// Represents a `Value` that can be turned into a [`Token`]
pub trait Value: Downcast + ::std::fmt::Debug + ::std::fmt::Display + Sync + Send {
    /// Gives the Value the opportunity to modify the [`Stack`] to its liking.
    /// If the `Value` performs no modifications it shall return `Err(Error::NotCallable)`
    /// and not try to push itself onto the [`Stack`] again, as this will lead to an infinite loop.
    fn apply(&self, stack: &mut Stack) -> Result<(), Error>;
}

impl_downcast!(Value);

pub trait Parser: ::std::fmt::Debug {
    /// Returns a `String`, which contains a regular expression as defined in the regex crate,
    /// that only matches a `str` if parsing it with `parse` would succeed
    fn parse_hint(&self) -> String;
    /// Parses `input` into a Token of this type
    /// # Panics
    /// Panics if the input can't be parsed into the associated type
    fn parse(&self, input: &str) -> Token {
        self.try_parse(input).unwrap()
    }
    /// Returns a parsed value by first matching the `str` with the `parse_hint`
    /// and then parsing it with `parse`
    /// # Panics
    /// Panics if `parse_hint` approves of an input but parse doesn't
    fn try_parse(&self, input: &str) -> Option<Token> {
        let re = Regex::new(&self.parse_hint()).unwrap();
        if re.is_match(input) {
            Some(self.parse(input))
        } else {
            None
        }
    }
}

#[derive(Debug)]
/// A parser, capable of turning str-representations of tokens into actual [`Token`]s.
/// The [`Parser`] of each type of [`Token`], which should be available in the `Parser`
/// have to be added to it before it can create such [`Token`]s.
pub struct ParserAggregator {
    objects: Vec<Box<dyn Parser>>,
}

impl ParserAggregator {
    /// Creates a new empty `Parser`, incapable of parsing anything
    pub fn new() -> ParserAggregator {
        ParserAggregator {
            objects: Vec::new(),
        }
    }
    /// Adds a [`Parser`] to the `Parser`.
    /// If the capabilities of multiple [`Parser`]s overlap, the newest one takes precedence.
    pub fn push<M: Parser + 'static>(&mut self, object: M) {
        self.objects.push(Box::new(object));
    }
    /// Applies the `Parser` to an input, returning a [`Token`], which can be pushed onto a [`Stack`]
    pub fn try_parse(&self, input: &str) -> Option<Token> {
        self.objects
            .iter()
            .rev()
            .filter_map(|object| object.try_parse(input))
            .next()
    }
}

impl Parser for ParserAggregator {
    fn parse_hint(&self) -> String {
        let mut re_string = String::new();
        for type_object in &self.objects {
            re_string.push('(');
            re_string.push_str(type_object.parse_hint().as_str());
            re_string.push_str(")|");
        }
        re_string.pop();
        re_string
    }
    fn try_parse(&self, input: &str) -> Option<Token> {
        ParserAggregator::try_parse(self, input)
    }
}
