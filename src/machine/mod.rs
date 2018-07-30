use downcast_rs::Downcast;
use regex::Regex;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "not enough values on the stack")]
    NotEnoughValues,
    #[fail(display = "token is not the expected type: {:?}", token,)]
    WrongType { token: Token },
    #[fail(display = "token is not an operator: {:?}", token)]
    NotCallable { token: Token },
}
#[derive(Debug)]
pub struct Token(Box<dyn Value>);

impl Token {
    pub fn new<T: Value + 'static>(value: T) -> Token {
        Token(Box::new(value))
    }
    pub fn apply(self, stack: &mut Stack) -> Result<(), Error> {
        self.0.apply(stack)
    }
    pub fn downcast<T: Value + 'static>(self) -> Option<Box<T>> {
        self.0.downcast().ok()
    }
    pub fn downcast_ref<T: Value + 'static>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

#[derive(Debug)]
pub struct Stack(Vec<Token>);

impl Stack {
    pub fn new() -> Stack {
        Stack(Vec::new())
    }
    pub fn push(&mut self, t: Token) -> Result<(), Error> {
        match t.apply(self) {
            Err(Error::NotCallable { token }) => Ok(self.0.push(token)),
            n => n,
        }
    }

    pub fn pop(&mut self) -> Result<Token, Error> {
        self.0.pop().ok_or(Error::NotEnoughValues)
    }
}

pub trait Value: Downcast + ::std::fmt::Debug + Sync + Send {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error>;
}

impl_downcast!(Value);

pub trait MetaObject: ::std::fmt::Debug {
    /// Returns a `Regex`, that only matches a `str` if parsing it with `parse` would succeed
    fn parse_hint(&self) -> Regex;
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
        let re = self.parse_hint();
        if re.is_match(input) {
            Some(self.parse(input))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    objects: Vec<Box<dyn MetaObject>>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            objects: Vec::new(),
        }
    }
    pub fn push<M: MetaObject + 'static>(&mut self, object: M) {
        self.objects.push(Box::new(object));
    }
    pub fn try_parse(&self, input: &str) -> Option<Token> {
        self.objects
            .iter()
            .rev()
            .filter_map(|object| object.try_parse(input))
            .next()
    }
}
