use crate::parsearg::IR;
use std::env;
use std::io::Read;
use std::process;
pub mod parsearg;

fn main() {
    println!("Symbolic, the language that has no keywords");
    let mut args = parsearg::Args::from_args(env::args()).unwrap_or_else(|_err| {
        println!("Invalid args");
        process::exit(1);
    });

    println!(
        "{} {}...",
        match args.to {
            IR::Output => "Running",
            IR::AST => "Parsing",
            IR::Bytecode => "Compiling",
            IR::TokenStream => "Tokenizing",
            IR::Src => {
                panic!("What's the fxxk")
            }
        },
        args.input.name,
    );
    let mut data = Default::default();
    args.input.read_to_string(&mut data).unwrap_or_else(|err| {
        println!("Error reading file: {}", err);
        process::exit(1);
    });
    println!("Read:\n{}", data)
}
