#[macro_use]
extern crate failure;

#[macro_use]
extern crate downcast_rs;

pub mod arithmetic;
pub mod machine;

#[cfg(test)]
mod tests {
    use arithmetic::*;
    use machine::*;
    #[test]
    fn simple_addition() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(4)));
        s.push(Token::new(Integer(5)));
        s.push(Token::new(Addition));
        s.step().unwrap();
        assert_eq!(Integer(9), *s.pop().unwrap().downcast().unwrap());
    }
    #[test]
    fn simple_multiplication() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(4)));
        s.push(Token::new(Integer(5)));
        s.push(Token::new(Multiplication));
        s.step().unwrap();
        assert_eq!(Integer(20), *s.pop().unwrap().downcast().unwrap());
    }
    #[test]
    fn simple_substraction() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(4)));
        s.push(Token::new(Integer(5)));
        s.push(Token::new(Substraction));
        s.step().unwrap();
        assert_eq!(Integer(-1), *s.pop().unwrap().downcast().unwrap());
    }
    #[test]
    fn simple_division() {
        let mut s = Stack::new();
        s.push(Token::new(Integer(20)));
        s.push(Token::new(Integer(5)));
        s.push(Token::new(Division));
        s.step().unwrap();
        assert_eq!(Integer(4), *s.pop().unwrap().downcast().unwrap());
    }
}
