extern crate failure;
extern crate stack_calculator;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use failure::Error;
use stack_calculator::arithmetic::*;
use stack_calculator::machine::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "stack-calculator")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: ::std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let opts = Opt::from_args();
    println!("{:?}", opts);
    let mut stack = parse(::std::fs::File::open(opts.file)?)?;
    println!("{:?}", stack);
    println!("{:?}", stack.collapse());
    Ok(())
}

use std::io::Read;

fn parse<I: Read>(mut i: I) -> Result<Stack, Error> {
    let mut stack = Stack::new();
    let mut input = String::new();
    i.read_to_string(&mut input)?;
    let words = input.split_whitespace();
    for word in words {
        stack.push(match word {
            "+" => Token::new(Addition),
            "-" => Token::new(Substraction),
            "*" => Token::new(Multiplication),
            "/" => Token::new(Division),
            num => Token::new(Integer(num.parse()?)),
        })?;
    }
    Ok(stack)
}
