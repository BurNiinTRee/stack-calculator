extern crate failure;
extern crate rustyline;
extern crate stack_calculator;
#[macro_use]
extern crate structopt;

use structopt::StructOpt;

use failure::Error;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use stack_calculator::arithmetic::*;
use stack_calculator::machine::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(StructOpt, Debug)]
#[structopt(name = "stack-calculator")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    file: Option<::std::path::PathBuf>,
    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

fn main() -> Result<(), Error> {
    let opts = Opt::from_args();
    let parser = arithmetic_module();
    let stack = Stack::new();
    match opts.file {
        Some(path) => exec_file(stack, parser, path, opts.debug),
        None => exec_stdin(stack, parser),
    }?;
    Ok(())
}

fn exec_stdin(mut stack: Stack, parser: Parser) -> Result<Stack, Error> {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                for word in line.split_whitespace() {
                    stack.push(match parser.try_parse(word) {
                        Some(n) => n,
                        None => {
                            println!("Couldn't parse {}, ignoring", word);
                            continue;
                        }
                    })?;
                }
                println!("{}", stack);
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            err => {
                err?;
            }
        }
    }
    Ok(stack)
}

fn exec_file<P: AsRef<::std::path::Path>>(
    mut stack: Stack,
    parser: Parser,
    path: P,
    debug: bool,
) -> Result<Stack, Error> {
    let input = BufReader::new(File::open(path)?);
    for line in input.lines() {
        for word in line?.split_whitespace() {
            let token = parser
                .try_parse(word)
                .ok_or_else(|| failure::err_msg(format!("couldn't parse {}", word)))?;
            if debug {
                println!("{:?}", token);
            }
            stack.push(token)?;
            if debug {
                println!("{}", stack);
            }
        }
    }
    Ok(stack)
}
