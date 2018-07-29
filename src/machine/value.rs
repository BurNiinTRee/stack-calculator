use super::{Stack, Token};
use downcast_rs::Downcast;

pub trait Value: Downcast + ::std::fmt::Debug + Sync + Send {
    fn apply(&self, stack: &mut Stack) -> Result<(), Error>;
}

impl_downcast!(Value);

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "not enough values on the stack")]
    NotEnoughValues,
    #[fail(display = "token is not the expected type: {:?}", token,)]
    WrongType { token: Token },
    #[fail(display = "token is not an operator: {:?}", token)]
    NotCallable { token: Token },
}
