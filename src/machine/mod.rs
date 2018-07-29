mod value;
pub use self::value::*;

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
