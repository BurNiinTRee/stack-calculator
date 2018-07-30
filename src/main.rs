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
    let mut stack = exec(::std::fs::File::open(opts.file)?)?;
    println!("{:?}", stack);
    println!("{:?}", stack.pop());
    Ok(())
}

use std::io::Read;

fn exec<I: Read>(mut i: I) -> Result<Stack, Error> {
    let mut parser = Parser::new();
    parser.push(AdditionMeta);
    parser.push(SubstractionMeta);
    parser.push(MultiplicationMeta);
    parser.push(DivisionMeta);
    parser.push(IntegerMeta);
    let mut stack = Stack::new();
    let mut input = String::new();
    i.read_to_string(&mut input)?;
    let words = input.split_whitespace();
    for word in words {
        stack.push(
            parser
                .try_parse(word)
                .ok_or_else(|| failure::err_msg(format!("Couldn't parse {}", word)))?,
        )?;
    }
    Ok(stack)
}
